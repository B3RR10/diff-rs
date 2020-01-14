//! The initial point is to parse the arguments, if exists and fetch the stdin
//! throw the parser to print the diff content in a beautiful way.

mod file;
mod parser;
mod printer;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::io::{self, Read, Write};
use strip_ansi_escapes;

fn main() -> io::Result<()> {
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
    let plain_buffer = String::from_utf8(strip_ansi_escapes::strip(&buffer).unwrap()).unwrap();

    if plain_buffer.starts_with("diff --git") {
        let files: Vec<file::File> = parser::parse_content(&plain_buffer);

        println!("{}", printer::print(&files, columnview));

        Ok(())
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write_all(buffer.as_bytes())?;

        Ok(())
    }
}
