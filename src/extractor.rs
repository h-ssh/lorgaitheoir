#[allow(clippy::collapsible_if)] // tempfix for cicd - remove later
#[allow(clippy::new_without_default)] // tempfix for cicd - remove later
use crate::types::{Chunk, Feature};
use regex::Regex;

pub trait Extractor {
    fn name(&self) -> &'static str;
    fn extract(&self, chunk: &Chunk) -> Vec<Feature>;
}

pub struct KeywordExtractor {
    keyword: String,
}

impl KeywordExtractor {
    pub fn new(keyword: String) -> Self {
        Self { keyword }
    }
}

impl Extractor for KeywordExtractor {
    fn name(&self) -> &'static str {
        "keyword"
    }

    fn extract(&self, chunk: &Chunk) -> Vec<Feature> {
        let mut results = Vec::new();

        if let Ok(text) = std::str::from_utf8(&chunk.data) {
            if text.contains(&self.keyword) {
                results.push(Feature {
                    kind: format!("keyword('{}')", self.keyword),
                    offset: chunk.offset,
                    data: chunk.data.clone(),
                });
            }
        }

        results
    }
}

pub struct EmailExtractor {
    pattern: Regex,
}

impl EmailExtractor {
    pub fn new() -> Self {
        Self {
            pattern: Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap(),
        }
    }
}

impl Extractor for EmailExtractor {
    fn name(&self) -> &'static str {
        "email"
    }

    fn extract(&self, chunk: &Chunk) -> Vec<Feature> {
        let mut results = Vec::new();

        if let Ok(text) = std::str::from_utf8(&chunk.data) {
            for mat in self.pattern.find_iter(text) {
                results.push(Feature {
                    kind: format!("email('{}')", mat.as_str()),
                    offset: chunk.offset + mat.start() as u64,
                    data: mat.as_str().as_bytes().to_vec(),
                });
            }
        }

        results
    }
}
