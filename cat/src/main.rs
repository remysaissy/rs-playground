#[macro_use]
extern crate clap;

use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader, Lines, Result};
use std::path::Path;

use clap::{App, ArgMatches};

fn main() -> Result<()> {
    let yaml = load_yaml!("usage.yml");
    let options = App::from_yaml(yaml).get_matches();
    match options.values_of("FILE") {
        Some(files) => {
            for filename in files {
                print_file(filename, &options);
            }
        }
        None => print_file("-", &options),
    };
    Ok(())
}

fn print_file(filename: &str, options: &ArgMatches) {
    let show_ends = match options.is_present("show_ends") {
        true => "$",
        false => "",
    };
    match filename {
        "-" => {
            let lines = io::BufReader::new(std::io::stdin()).lines();
            for line in lines {
                eprintln!("{}{}", line.unwrap(), show_ends);
            }
        }
        _ => {
            match read_file(filename) {
                Ok(lines) => {
                    for line in lines {
                        eprintln!("{}{}", line.unwrap(), show_ends);
                    }
                }
                Err(e) => eprintln!("{}", e),
            };
        }
    };
}

fn read_file<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
