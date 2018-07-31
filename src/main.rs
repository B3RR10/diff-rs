use std::io;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;

fn main() {

    // create cli app
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("A rust text diffing and assertion library.")
        .long_about("A more beautiful and readable diff output.")
        .arg(
            Arg::with_name("SOURCE")
                .help("Set the source file for diff")
                .long_help(
                    "Set a path to the source file to diff with another file",
                )
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DESTINATION")
                .help("Set the destination file for diff")
                .long_help(
                    "Set a path to the destination file to diff with the source file",
                )
                .index(2)
                .takes_value(true),
        )
        .arg(Arg::with_name("columnview")
            .short("c")
            .long("column")
            .help("Show in 2 columnview")
        ).get_matches();

    let column_view = matches.value_of("columnview").unwrap_or("");
    let source_file = matches.value_of("source").unwrap_or("");
    let destination_file = matches.value_of("destination").unwrap_or("");

//    println!("SOURCE {:?}", source_file);
//    println!("DEST {:?}", destination_file);
//    println!("COLUMN {:?}", column_view);

    let stdin = io::stdin();

    let mut lines: Vec<String> = vec![];
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    differ(&lines);
}

fn differ(line: &Vec<String>) {
    line.iter().for_each(|f| println!("{}", f));
}

// Test cases
use std::path::PathBuf;

#[test]
fn test_with_diff_file() {
    // load test/resources/diff.patch file for output test
    let mut test_diff_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_diff_path.push("test/resources/diff.patch");
    println!("Read file: {:?}", test_diff_path);

    let mut diff_file = File::open(test_diff_path).expect("file not found");

    let mut diff_content = String::new();
    diff_file.read_to_stri	g(&mut diff_content)
        .expect("something went wrong reading the file");

    println!("File content:\n{}", diff_content);
    assert!(true);
}
