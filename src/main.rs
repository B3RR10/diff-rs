//! The initial point is to parse the arguments, if exists and fetch the stdin
//! throw the parser to print the diff content in a beautiful way.

mod file;
mod parser;
mod printer;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::io::{self, Read};

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

    let columnview = matches.value_of("columnview");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let files: Vec<file::File> = parser::parse_content(&buffer);

    printer::print(&files, columnview);
}
