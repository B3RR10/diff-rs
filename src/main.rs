//! The initial point is to parse the arguments, if exists and fetch the stdin
//! throw the parser to print the diff content in a beautiful way.

mod file;
mod parser;
mod printer;

#[macro_use]
extern crate clap;

use file::*;
use parser::*;

use clap::{App, Arg};
use std::io;
use std::io::BufRead;

fn main() {
    // create cli app
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .long_about(crate_description!())
        .arg(
            Arg::with_name("columnview")
                .short("c")
                .long("column")
                .help("Show in 2 columnview"),
        )
        .get_matches();

    let column_view = matches.value_of("columnview");

    let stdin = io::stdin();

    let mut lines: Vec<String> = vec![];
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        lines.push(line);
    }

    let files: Vec<file::File> = parse_content(&lines);

    printer::print(&files);
}

// Test cases
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[test]
fn test_with_diff_file() {
    // load test/resources/diff.patch file for output test
    let mut test_diff_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_diff_path.push("test/resources/diff.patch");
    println!("Read file: {:?}", test_diff_path);

    let mut diff_file = File::open(test_diff_path).expect("file not found");

    let mut diff_content = String::new();
    diff_file
        .read_to_string(&mut diff_content)
        .expect("something went wrong reading the file");

    println!("File content:\n{}", diff_content);
    assert!(true);
}

#[test]
fn test_print_file() {
    let mut files = vec![file::File::new(
        MODIFIER::MODIFIED,
        "Cargo.toml".to_string(),
        "5ac01d12897f32eabe8839af95ae446209e815ab".to_string(),
        vec![Hunk::new(
            vec![
                Line {
                    modifier: MODIFIER::NOP,
                    line_number: 1,
                    line: String::from(" #!/usr/bin/env bash"),
                },
                Line {
                    modifier: MODIFIER::NOP,
                    line_number: 2,
                    line: String::from(" "),
                },
                Line {
                    modifier: MODIFIER::DELETE,
                    line_number: 3,
                    line: String::from("-echo \"Test\""),
                },
                Line {
                    modifier: MODIFIER::ADD,
                    line_number: 3,
                    line: String::from("+echo \"Test is going on\""),
                },
            ]
            .iter()
            .cloned()
            .collect(),
        )],
    )];

    printer::print(&files);
}
