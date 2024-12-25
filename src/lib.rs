use std::error::Error;
use std::{fs::File, io::Read};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let cnt = concatenate(&content);
    println!("{}", cnt);
    Ok(())
}

fn concatenate(content: &str) -> &str {
    content
}
pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
        let filename = match args.skip(1).next() {
            Some(filename) => filename,
            None => return Err("couldn't parse filename"),
        };

        Ok(Config { filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat() {
        let content = "\
The Legend
Himself.";

        assert_eq!("The Legend\nHimself.", concatenate(content))
    }
}
