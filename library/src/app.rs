use super::config::Config;

use std::{error::Error, fs};

use colored::Colorize;
use regex::Regex;

pub struct App {
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> App {
        App { config }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let query = Regex::new(&self.config.query).unwrap();

        if self.config.file_path.is_dir() {
            self.handle_directories(&query)
        } else {
            self.handle_files(&query)
        }
    }

    fn handle_directories<'a>(&mut self, query: &'a Regex) -> Result<(), Box<dyn Error>> {
        let dir = fs::read_dir(&self.config.file_path).unwrap();

        for file in dir.collect::<Vec<_>>().into_iter().flatten() {
            if file.path().is_dir() {
                self.config.file_path = file.path();
                let _ = self.run();
            } else {
                let content = fs::read_to_string(file.path())?;

                let results = self.search(&query, &content);

                if !results.is_empty() {
                    println!("{}", file.path().display());
                    for result in results {
                        println!("{}", self.highlight(&query, result));
                    }

                    println!();
                }
            }
        }

        Ok(())
    }

    fn handle_files<'a>(&self, query: &'a Regex) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&self.config.file_path)?;

        for result in self.search(&query, &content) {
            println!("{}", self.highlight(&query, result));
        }

        Ok(())
    }

    fn search<'a>(&self, query: &'a Regex, content: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in content.lines() {
            if query.is_match(line) {
                results.push(line)
            }
        }

        results
    }

    fn highlight<'a>(&self, query: &'a Regex, line: &'a str) -> String {
        let mut result = String::from(line);

        for letter in query.captures(line).unwrap().iter().flatten() {
            result = result.replace(letter.as_str(), &letter.as_str().red().to_string());
        }

        result
    }
}
