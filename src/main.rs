extern crate rustycat;

use rustycat::Config;

use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("error parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = rustycat::run(config) {
        eprintln!("error while running application: {}", e);
        process::exit(1);
    }
}
