use super::{AssetSubset, DedupSummary, DuplicateAsset, IdenticalAssetGroup, InventoryAsset};
use serde::Serialize;
use std::collections::HashMap;

struct AssetClass {
    files: Vec<usize>,
    bytes: u64,
    members: Vec<DuplicateAsset>,
}
struct Relation {
    smaller: usize,
    larger: usize,
    end: usize,
}

pub(super) struct AssetRelations {
    pub identical: Vec<IdenticalAssetGroup>,
    classes: Vec<AssetClass>,
    relations: Vec<Relation>,
}

impl AssetRelations {
    pub fn build(inventory: Vec<InventoryAsset>, sizes: &[u64], summary: &mut DedupSummary) -> Self {
        let mut by_set: HashMap<Vec<usize>, Vec<DuplicateAsset>> = HashMap::new();
        for mut asset in inventory {
            if asset.total == 0 || asset.hashed != asset.total {
                continue;
            }
            summary.complete_assets += 1;
            asset.payload.sort_unstable();
            asset.payload.dedup();
            by_set.entry(asset.payload).or_default().push(asset.asset);
        }
        let mut classes: Vec<_> = by_set
            .into_iter()
            .map(|(files, mut members)| {
                members.sort_unstable_by(|a, b| a.rel_dir.cmp(&b.rel_dir));
                let bytes = files.iter().map(|&key| sizes[key]).sum();
                AssetClass {
                    files,
                    bytes,
                    members,
                }
            })
            .collect();
        classes.sort_unstable_by(|a, b| {
            b.bytes
                .cmp(&a.bytes)
                .then_with(|| a.members[0].rel_dir.cmp(&b.members[0].rel_dir))
        });

        let mut identical = Vec::new();
        let mut postings = vec![Vec::new(); sizes.len()];
        for (class, c) in classes.iter().enumerate() {
            for &file in &c.files {
                postings[file].push(class)
            }
            if c.members.len() > 1 {
                summary.identical_groups += 1;
                summary.identical_assets += c.members.len();
                identical.push(IdenticalAssetGroup {
                    unique_files: c.files.len(),
                    unique_bytes: c.bytes,
                    assets: c.members.clone(),
                });
            }
        }

        // Every strict superset must appear in the rarest hash's posting list. Test only these
        // candidates. Identical sets share one node; their Cartesian product is expanded on demand.
        let mut relations = Vec::new();
        for (smaller_id, smaller) in classes.iter().enumerate() {
            let Some(&rarest) = smaller.files.iter().min_by_key(|&&key| postings[key].len()) else {
                continue;
            };
            for &larger_id in &postings[rarest] {
                let larger = &classes[larger_id];
                if larger.files.len() > smaller.files.len()
                    && smaller
                        .files
                        .iter()
                        .all(|key| larger.files.binary_search(key).is_ok())
                {
                    summary.subset_pairs += smaller.members.len() * larger.members.len();
                    relations.push(Relation {
                        smaller: smaller_id,
                        larger: larger_id,
                        end: summary.subset_pairs,
                    });
                }
            }
        }
        Self {
            identical,
            classes,
            relations,
        }
    }

    pub fn pairs_from(&self, offset: usize) -> impl Iterator<Item = AssetSubset> + '_ {
        let start = self.relations.partition_point(|r| r.end <= offset);
        self.relations[start..].iter().flat_map(move |r| {
            let smaller = &self.classes[r.smaller];
            let larger = &self.classes[r.larger];
            let n = smaller.members.len() * larger.members.len();
            let skip = offset.saturating_sub(r.end - n);
            (skip..n).map(move |i| AssetSubset {
                smaller: smaller.members[i / larger.members.len()].clone(),
                larger: larger.members[i % larger.members.len()].clone(),
                smaller_files: smaller.files.len(),
                larger_files: larger.files.len(),
            })
        })
    }
}

impl Serialize for AssetRelations {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.pairs_from(0))
    }
}
