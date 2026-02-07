use regex::Regex;
use clap::{App,Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let contains_substring = re.find(&line);
        match contains_substring { 
            Some(_) => println!("{}", line), 
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("seraches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("file")
            .help("The file to search through")
            .takes_value(true)
            .required(false))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let file_name = args.value_of("file").unwrap_or("-");

    if file_name == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let stdin = File::open(file_name).unwrap();
        let reader = BufReader::new(stdin);
        process_lines(reader, re);
    }
}
