#[macro_use]
extern crate lazy_static;
mod scanner;
use std::env;
use std::fs::File;
use std::path::Path;
pub use crate::scanner::scanner::file_scan;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", "Missing source file!");
        println!("{}", "Usage: Lexer.exe <Source File> [Output File]\nSource file is required. Output File is \"tokenOut.txt\" by default.");
        return;
    }

    if args.len() > 3 {
        println!("{}", "Too many Arguments!");
        println!("{}", "Usage: Lexer.exe <Source File> [Output File]\nSource file is required. Output File is \"tokenOut.txt\" by default.");
        return;
    }

    let input_path = Path::new(&args[1]);

    let output_path;

    if args.len() == 3 {
        output_path = Path::new(&args[2]);
    } else {
        output_path = Path::new("./TokenOut.txt");
    }

    let mut input_file = match File::open(&input_path) {
        Err(why) => panic!("Can not open {}: {}!", input_path.display(), 
                                                   why),
        Ok(input_file) => input_file,
    };

    let mut output_file = match File::create(&output_path) {
        Err(why) => panic!("Can not create {}: {}!", output_path.display(), 
                                                   why),
        Ok(output_file) => output_file,
    };

    file_scan(&mut input_file, &mut output_file);
}

