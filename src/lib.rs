use std::error::Error;
use std::{fs::File, io::Read};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let cnt = concatenate(&content);
    println!("{}", cnt);

    match config.search {
        Some(x) => println!("{}", x),
        None => println!("No value passed to search"),
    }
    Ok(())
}

fn concatenate(content: &str) -> &str {
    content
}

pub struct Config {
    pub filename: String,
    pub search: Option<String>,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("filename not provided"),
        };

        let mut search = None;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" => {
                    search = match args.next() {
                        Some(val) => Some(val),
                        None => return Err("search string not provided after -s flag"),
                    }
                }
                _ => return Err("invalid argument provided"),
            }
        }

        Ok(Config { filename, search })
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
