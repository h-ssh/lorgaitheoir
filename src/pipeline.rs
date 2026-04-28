use crate::{
    config::Config,
    extractor::{EmailExtractor, Extractor, KeywordExtractor},
    output,
    scanner::FileScanner,
};

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let scanner = FileScanner::new(&config.input, config.chunk_size)?;

    let mut extractors: Vec<Box<dyn Extractor>> = Vec::new();
    for scanner in config.scanners {
        let extractor: Box<dyn Extractor> = match scanner {
            crate::config::Scanner::Keyword(keyword) => Box::new(KeywordExtractor::new(keyword)),
            crate::config::Scanner::Email => Box::new(EmailExtractor::new()),
        };
        extractors.push(extractor);
    }

    for chunk in scanner {
        for extractor in &extractors {
            let features = extractor.extract(&chunk);
            output::write(features);
        }
    }

    Ok(())
}
