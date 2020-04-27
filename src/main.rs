mod token;
mod dictionary;
mod scanner;
use crate::token::*;
use crate::scanner::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please enter file name!");
    }

    if args.len() > 3 {
        panic!("Many argument!");
    }

    let mut file;
    let file_name;

    if args.len() == 3 {
        file_name = args[2].to_owned();
    } else {
        file_name = "./TokenOut.txt".to_string();
    }

    file = match File::create(file_name) {
        Err(err) => panic!("Can not create {}, because {}.", args[2], err.to_string()),
        Ok(file) => file,
    };

    let mut scanner = Scanner::new(args[1].to_owned());

    loop {
        match scanner.get_token().get_token_type() {
            TokenType::END_OF_FILE => break,
            _ => {},
        }

        file.write_all(format!("{}\n",scanner.get_next_token().to_string()).as_bytes()).unwrap();
    }
}
