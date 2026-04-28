use crate::types::Chunk;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct FileScanner {
    reader: BufReader<File>,
    offset: u64,
    chunk_size: usize,
}

impl FileScanner {
    pub fn new(path: &str, chunk_size: usize) -> std::io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
            offset: 0,
            chunk_size,
        })
    }
}

impl Iterator for FileScanner {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0; self.chunk_size];
        let n = self.reader.read(&mut buffer).ok()?;

        if n == 0 {
            return None;
        }

        buffer.truncate(n);

        let chunk = Chunk {
            offset: self.offset,
            data: buffer,
        };

        self.offset += n as u64;
        Some(chunk)
    }
}
