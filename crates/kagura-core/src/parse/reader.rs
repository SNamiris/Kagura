use std::{
    fs::File,
    io::{BufReader, Read, Seek},
};

use encoding_rs::SHIFT_JIS;

use super::{open_error, ParseError};

pub(super) type Result<T> = std::result::Result<T, ParseError>;

pub(super) struct Reader {
    input: BufReader<File>,
    length: u64,
    pub offset: u64,
    pub section: &'static str,
    text_budget: usize,
}

impl Reader {
    pub fn new(mut file: File) -> Result<Self> {
        let length = file.metadata().map_err(open_error)?.len();
        file.rewind().map_err(open_error)?;
        Ok(Self {
            input: BufReader::with_capacity(128 * 1024, file),
            length,
            offset: 0,
            section: "header",
            text_budget: 64 * 1024 * 1024,
        })
    }

    pub fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            section: self.section,
            offset: self.offset,
            message: message.into(),
        }
    }

    pub fn remaining(&self) -> u64 {
        self.length - self.offset
    }

    pub fn ensure(&self, bytes: u64) -> Result<()> {
        if bytes > self.remaining() {
            return Err(self.error(format!(
                "truncated: need {bytes} bytes, {} available",
                self.remaining()
            )));
        }
        Ok(())
    }

    pub fn skip(&mut self, bytes: u64) -> Result<()> {
        self.ensure(bytes)?;
        let distance = i64::try_from(bytes).map_err(|_| self.error("offset exceeds seek range"))?;
        self.input
            .seek_relative(distance)
            .map_err(|e| self.error(e.to_string()))?;
        self.offset += bytes;
        Ok(())
    }

    pub fn rows(&mut self, count: u32, size: u64) -> Result<()> {
        self.skip(u64::from(count) * size)
    }

    fn read(&mut self, dst: &mut [u8]) -> Result<()> {
        self.ensure(dst.len() as u64)?;
        self.input
            .read_exact(dst)
            .map_err(|e| self.error(e.to_string()))?;
        self.offset += dst.len() as u64;
        Ok(())
    }

    pub fn bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut data = [0; N];
        self.read(&mut data)?;
        Ok(data)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes::<1>()?[0])
    }
    pub fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.bytes()?))
    }
    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.bytes()?))
    }
    pub fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.bytes()?))
    }

    /// PMX counts and text lengths are signed 32-bit values.
    pub fn count(&mut self) -> Result<u32> {
        let n = i32::from_le_bytes(self.bytes()?);
        u32::try_from(n).map_err(|_| self.error("negative length or count"))
    }

    pub fn flag(&mut self) -> Result<bool> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(self.error(format!("invalid flag {other}"))),
        }
    }

    pub fn triangles(&mut self, indices: u32, width: u64) -> Result<u32> {
        if !indices.is_multiple_of(3) {
            return Err(self.error("face index count is not divisible by 3"));
        }
        self.rows(indices, width)?;
        Ok(indices / 3)
    }

    pub fn pmx_text(&mut self, encoding: &'static encoding_rs::Encoding) -> Result<String> {
        let len = self.count()? as usize;
        self.ensure(len as u64)?;
        // Untrusted length prefixes must never allocate the entire model or an unbounded names list.
        if len > 8 * 1024 * 1024 {
            return Err(self.error("text field exceeds 8 MiB"));
        }
        self.text_budget = self
            .text_budget
            .checked_sub(len + size_of::<String>())
            .ok_or_else(|| self.error("model text exceeds 64 MiB"))?;
        let mut raw = vec![0; len];
        self.read(&mut raw)?;
        Ok(encoding.decode_without_bom_handling(&raw).0.into_owned())
    }

    pub fn skip_text(&mut self) -> Result<()> {
        let len = self.count()?;
        self.skip(len.into())
    }

    pub fn sjis<const N: usize>(&mut self) -> Result<String> {
        Ok(decode_sjis(&self.bytes::<N>()?))
    }
}

pub(super) fn unpad(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|&c| c == 0).unwrap_or(bytes.len())]
}

pub(super) fn decode_sjis(bytes: &[u8]) -> String {
    SHIFT_JIS.decode_without_bom_handling(unpad(bytes)).0.into_owned()
}
