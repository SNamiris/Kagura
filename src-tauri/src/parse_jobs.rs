use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use kagura_core::index::{Index, ParseProgress};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Runtime};

use crate::desktop::{display, ApiError, Reply};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[serde(rename_all = "lowercase")]
pub(crate) enum ParsePhase {
    #[default]
    Idle,
    Running,
    Complete,
    Failed,
}

#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
pub(crate) struct ParseStatus {
    pub root: Option<String>,
    pub generation: u32,
    pub phase: ParsePhase,
    pub total: usize,
    pub cached: usize,
    pub processed: usize,
    pub failed: usize,
    pub error: Option<String>,
}

impl ParseStatus {
    fn counts(&mut self, p: &ParseProgress) {
        self.total = p.total;
        self.cached = p.cached;
        self.processed = p.processed;
        self.failed = p.failed;
    }
}

#[derive(Default)]
pub(crate) struct ParseJobs(Arc<Mutex<ParseStatus>>);

impl ParseJobs {
    pub fn status(&self) -> Reply<ParseStatus> {
        self.0
            .lock()
            .map(|status| status.clone())
            .map_err(|_| ApiError::new("worker_error", "解析状态不可用"))
    }

    pub fn start<R: Runtime>(&self, app: AppHandle<R>, root: PathBuf, database: PathBuf) -> Reply<()> {
        let state = self.0.clone();
        let generation = {
            let mut state = state
                .lock()
                .map_err(|_| ApiError::new("worker_error", "解析状态不可用"))?;
            let generation = state.generation.wrapping_add(1);
            *state = ParseStatus {
                root: Some(display(&root)),
                generation,
                phase: ParsePhase::Running,
                ..Default::default()
            };
            let _ = app.emit("parse-progress", &*state);
            generation
        };
        tauri::async_runtime::spawn_blocking(move || {
            // This connection owns its short write transactions. Browsing keeps its own WAL reader.
            let mut last_event = None::<Instant>;
            let result = Index::open_for_library(database, &root).and_then(|mut index| {
                index.parse_files(&root, |p| {
                    let Ok(mut status) = state.lock() else {
                        return false;
                    };
                    if status.generation != generation {
                        return false;
                    }
                    status.counts(&p);
                    if last_event.is_none_or(|t| t.elapsed() >= Duration::from_millis(250)) {
                        let _ = app.emit("parse-progress", &*status);
                        last_event = Some(Instant::now());
                    }
                    true
                })
            });
            if let Ok(mut status) = state.lock() {
                if status.generation != generation {
                    return;
                }
                match result {
                    Ok(p) => {
                        status.counts(&p);
                        status.phase = ParsePhase::Complete;
                    }
                    Err(e) => {
                        status.phase = ParsePhase::Failed;
                        status.error = Some(e.to_string());
                    }
                }
                let _ = app.emit("parse-progress", &*status);
            }
        });
        Ok(())
    }
}
