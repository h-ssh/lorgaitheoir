use lorgaitheoir::{config::Config, pipeline};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let config = match Config::from_args(&args) {
        Ok(cfg) => cfg,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    };

    pipeline::run(config)
}
