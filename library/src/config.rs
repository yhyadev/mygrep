use std::path::PathBuf;

pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Not enough arguments".to_string());
        }

        let query = args[1].clone();
        let file_path = PathBuf::from(&args[2]);

        Ok(Config { query, file_path })
    }
}
