pub enum Scanner {
    Keyword(String),
    Email,
}

pub struct Config {
    pub input: String,
    pub chunk_size: usize,
    pub scanners: Vec<Scanner>,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut scanners = Vec::new();
        let mut input = None;
        let mut index = 0;

        while index < args.len() {
            match args[index].as_str() {
                "-h" | "--help" => return Err(Self::usage()),
                "-kw" | "--keyword" => {
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        "error: missing keyword after -kw\n\n".to_string() + &Self::usage()
                    })?;
                    scanners.push(Scanner::Keyword(value.clone()));
                }
                "-e" | "--email" => {
                    scanners.push(Scanner::Email);
                }
                arg => {
                    if input.is_some() {
                        return Err(
                            "error: multiple input paths provided\n\n".to_string() + &Self::usage()
                        );
                    }
                    input = Some(arg.to_string());
                }
            }

            index += 1;
        }

        if scanners.is_empty() {
            return Err(Self::usage());
        }

        let input =
            input.ok_or_else(|| "error: missing input file\n\n".to_string() + &Self::usage())?;

        Ok(Self {
            input,
            chunk_size: 4096,
            scanners,
        })
    }

    pub fn usage() -> String {
        let usage = "usage: lorg [scanner ...] <file>\n\n";
        let options = "options:\n  -kw, --keyword <word>  search for keyword in file\n  -e, --email            search for email addresses in file\n  -h, --help             show this help\n";
        usage.to_string() + options
    }
}
