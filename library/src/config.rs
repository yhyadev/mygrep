use std::path::PathBuf;

pub struct Config {
    pub query: String,
    pub file_path: PathBuf,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("You must specify a query")),
        };

        let file_path = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => return Err(String::from("The file to grep is required")),
        };

        Ok(Config { query, file_path })
    }
}
