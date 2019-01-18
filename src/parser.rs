//! The parser analyses the diff content and return the file(s) to the printer
//! The modified lines get the same line numbers in the hunk content.

use file::{File, Hunk, Line, LINEMOD, MODIFIER};

pub fn parse_content(_input: &String) -> Vec<File> {
    let mut content: Vec<Line> = vec![];

    // create sample content
    content.push(Line {
        modifier: LINEMOD::NOP,
        line_number: 1,
        line: String::from(" #!/usr/bin/env bash"),
    });
    content.push(Line {
        modifier: LINEMOD::NOP,
        line_number: 2,
        line: String::from(" "),
    });
    content.push(Line {
        modifier: LINEMOD::REM,
        line_number: 3,
        line: String::from("-echo \"Test\""),
    });
    content.push(Line {
        modifier: LINEMOD::ADD,
        line_number: 3,
        line: String::from("+echo \"Test is going on\""),
    });

    vec![
        File::new(
            MODIFIER::ADD,
            "filename.rs".to_string(),
            "2jhg2323".to_string(),
            vec![
                Hunk::new(content.clone()),
                Hunk::new(content.clone()),
                Hunk::new(content.clone()),
            ],
        ),
        File::new(
            MODIFIER::MODIFIED,
            "nextfile.rs".to_string(),
            "9812i1u23".to_string(),
            vec![Hunk::new(content.clone())],
        ),
    ]
}
