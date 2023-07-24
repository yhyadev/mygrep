use mygrep_lib::app::*;
use mygrep_lib::config::*;
use mygrep_lib::help_message;

use std::env::args;
use std::process::exit;

fn main() {
    if args().collect::<Vec<String>>().len() == 1 {
        help_message();
        exit(1);
    }

    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {}", err);
        exit(1);
    });

    let mut app = App::new(config);

    if let Err(err) = app.run() {
        eprintln!("{}: {}", app.config.file_path.display(), err);
        exit(1);
    };
}
