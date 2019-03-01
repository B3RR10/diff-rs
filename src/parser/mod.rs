//! The parser analyses the diff content and return the file(s) to the printer
//! The modified lines get the same line numbers in the hunk content.

mod raw_parser;

use self::raw_parser::{ parse_raw_files,  RawFile };
use file::File;

// TODO: Principal function
pub fn parse_content(_input: &String) -> Vec<File> {
    // TODO: Convert all &str in Strings in the raw_parser
   let _raw_files: Vec<RawFile> = parse_raw_files(_input);
    vec![]
}
