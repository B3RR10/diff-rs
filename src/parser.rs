//! Preprocess the input to convert it to raw structures

use crate::file::{File, Hunk, LINE, MODIFIER};

#[derive(Debug, PartialEq)]
enum RawLine<'a> {
    Left(&'a str),
    Right(&'a str),
    Both(&'a str),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum ExtendedHeader<'a> {
    ChMode((&'a str, &'a str)),
    Deleted,
    NewFile,
    CopyFile((&'a str, &'a str)),
    RenameFile((&'a str, &'a str)),
    SimilarityIndex(&'a str),
    DissimilarityIndex(&'a str),
    Index(&'a str),
}

#[derive(Debug, PartialEq)]
struct RawHeader<'a> {
    filenames: (&'a str, &'a str),
    extended_headers: Vec<ExtendedHeader<'a>>,
}

#[derive(Debug, PartialEq)]
struct RawHunk<'a> {
    line_info: (u32, u32, u32, u32),
    lines: Vec<RawLine<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct RawFile<'a> {
    header: RawHeader<'a>,
    hunks: Vec<RawHunk<'a>>,
}

#[allow(dead_code)]
fn is_space(c: char) -> bool {
    nom::is_space(c as u8)
}
#[allow(dead_code)]
fn is_new_line(c: char) -> bool {
    c == '\n'
}
#[allow(dead_code)]
fn is_whitespace(c: char) -> bool {
    is_space(c) || is_new_line(c)
}

// "diff --git a/script.sh b/script.sh\n"
named!(parse_filename(&str) -> (&str, &str), do_parse!(
        opt!(take_until_and_consume!("diff --git a/")) >>
        filename_a: take_until_and_consume!(" b/") >>
        filename_b: take_until_and_consume!("\n") >>
        ((filename_a, filename_b))
));

named!(parse_extended_header_mode(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("old mode ") >>
        old_mode: take_until_and_consume!("\n") >>
        tag!("new mode ") >>
        new_mode: take_until_and_consume!("\n") >>
        (ExtendedHeader::ChMode((old_mode, new_mode)))
));

named!(parse_extended_header_deleted(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("deleted") >> take_until_and_consume!("\n") >> (ExtendedHeader::Deleted)
));

named!(parse_extended_header_new_file(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("new file") >> take_until_and_consume!("\n") >> (ExtendedHeader::NewFile)
));

named!(parse_extended_header_copy_file(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("copy from ") >>
        from_path: take_until_and_consume!("\n") >>
        tag!("copy to ") >>
        to_path: take_until_and_consume!("\n") >>
        (ExtendedHeader::CopyFile((from_path, to_path)))
));

named!(parse_extended_header_rename_file(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("rename from ") >>
        from_path: take_until_and_consume!("\n") >>
        tag!("rename to ") >>
        to_path: take_until_and_consume!("\n") >>
        (ExtendedHeader::RenameFile((from_path, to_path)))
));

named!(parse_extended_header_similarity_index(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("similarity index ") >>
        index: take_until_and_consume!("\n") >>
        (ExtendedHeader::SimilarityIndex(index))
));

named!(parse_extended_header_dissimilarity_index(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("dissimilarity index ") >>
        index: take_until_and_consume!("\n") >>
        (ExtendedHeader::DissimilarityIndex(index))
));

named!(parse_extended_header_index(&str) -> ExtendedHeader<'_>, do_parse!(
        tag!("index") >>
        take_until_and_consume!("..") >>
        index: take_till!(is_whitespace) >>
        take_until_and_consume!("\n") >>
        (ExtendedHeader::Index(index))
));

named!(parse_extended_header(&str) -> ExtendedHeader<'_>, do_parse!(
        extended_header: alt!(
            parse_extended_header_mode | parse_extended_header_deleted |
            parse_extended_header_new_file | parse_extended_header_copy_file |
            parse_extended_header_rename_file | parse_extended_header_similarity_index |
            parse_extended_header_dissimilarity_index | parse_extended_header_index ) >>
        (extended_header)));

named!(parse_raw_file_header(&str) -> RawHeader<'_>, do_parse!(
        filenames: parse_filename >>
        extended_headers: many0!(complete!(parse_extended_header)) >>
        (RawHeader {
            filenames,
            extended_headers
        })
));

named!(parse_file_names_after_extended_header(&str) -> (), do_parse!(
        tag!("--- ") >> take_until_and_consume!("\n") >>
        tag!("+++ ") >> take_until_and_consume!("\n") >>
        ()
));

named!(parse_u32(&str) -> Result<u32, std::num::ParseIntError>,
    map!(nom::digit, std::str::FromStr::from_str)
);

// "@@ -1,3 +1,3 @@\n";
named!(parse_lines_info(&str) -> (u32, u32, u32, u32), do_parse!(
        opt!(parse_file_names_after_extended_header) >>
        tag!("@@ -") >>
        start_line_left: parse_u32 >>
        tag!(",") >>
        lines_left: parse_u32 >>
        tag!(" +") >>
        start_line_right: parse_u32 >>
        tag!(",") >>
        lines_right: parse_u32 >>
        take_until_and_consume!("\n") >>
        (start_line_left.unwrap(), lines_left.unwrap(), start_line_right.unwrap(), lines_right.unwrap())
));

named!(parse_line_both(&str) -> RawLine<'_>, do_parse!(
        tag!(" ") >>
        content: take_till!(is_new_line) >>
        (RawLine::Both(content))
));

named!(parse_line_left(&str) -> RawLine<'_>, do_parse!(
        tag!("-") >>
        content: take_till!(is_new_line) >>
        (RawLine::Left(content))
));

named!(parse_line_right(&str) -> RawLine<'_>, do_parse!(
        tag!("+") >>
        content: take_till!(is_new_line) >>
        (RawLine::Right(content))
));

named!(parse_line(&str) -> RawLine<'_>, do_parse!(
        line: alt!(parse_line_both | parse_line_left | parse_line_right) >>
        opt!(alt!(tag!("\n") | eof!())) >>
        (line)
));

named!(parse_lines(&str) -> Vec<RawLine<'_>>,
       many0!(complete!(parse_line))
);

named!(parse_raw_file_hunk(&str) -> RawHunk<'_>, do_parse!(
        line_info: parse_lines_info >>
        lines: parse_lines >>
        (RawHunk {
            line_info,
            lines
        })
));

named!(parse_raw_file(&str) -> RawFile<'_>, do_parse!(
        header: complete!(parse_raw_file_header) >>
        hunks: many0!(complete!(parse_raw_file_hunk)) >>
        (RawFile {
            header,
            hunks
        })
));

named!(parse_raw_files_intern(&str) -> Vec<RawFile<'_>>,
       many0!(complete!(parse_raw_file))
);

pub fn parse_raw_files<'a>(input: &'a str) -> Result<Vec<RawFile<'a>>, String> {
    match parse_raw_files_intern(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(format!(
                    "Error parsing file. Remaining is not empty. Remaining: {:?}",
                    remaining
                ))
            } else {
                Ok(result)
            }
        }
        Err(e) => Err(format!("Error parsing file. {}", e)),
    }
}

pub fn parse_content(input: &str) -> Vec<File> {
    let raw_files: Vec<RawFile<'_>> = parse_raw_files(input).unwrap();

    let mut parsed_files: Vec<File> = Vec::new();

    for raw_file in raw_files {
        let filename: String = raw_file.header.filenames.0.into();
        let mut commit_id: String = "".to_string();
        let mut modifier: MODIFIER = MODIFIER::MODIFIED;
        for extended_header in &raw_file.header.extended_headers {
            match extended_header {
                ExtendedHeader::Index(index) => commit_id = index.to_string(),
                ExtendedHeader::NewFile => modifier = MODIFIER::ADD,
                ExtendedHeader::Deleted => modifier = MODIFIER::DELETE,
                ExtendedHeader::RenameFile(_) => modifier = MODIFIER::RENAMED,
                _ => modifier = MODIFIER::MODIFIED,
            }
        }

        let mut hunks: Vec<Hunk> = Vec::new();
        for hunk in &raw_file.hunks {
            let mut lines: Vec<LINE> = Vec::new();
            let mut line_nr_left = hunk.line_info.0;
            let mut line_nr_right = hunk.line_info.2;

            for line in &hunk.lines {
                match line {
                    RawLine::Left(content) => {
                        lines.push(LINE::REM {
                            number: line_nr_left as usize,
                            line: String::from(*content),
                        });
                        line_nr_left += 1;
                    }
                    RawLine::Right(content) => {
                        lines.push(LINE::ADD {
                            number: line_nr_right as usize,
                            line: String::from(*content),
                        });
                        line_nr_right += 1;
                    }
                    RawLine::Both(content) => {
                        lines.push(LINE::NOP {
                            number_left: line_nr_left as usize,
                            number_right: line_nr_right as usize,
                            line: String::from(*content),
                        });
                        line_nr_right += 1;
                        line_nr_left += 1;
                    }
                }
            }
            hunks.push(Hunk::new(lines));
        }
        parsed_files.push(File::new(modifier, filename, commit_id, hunks))
    }
    parsed_files
}

/* --------------------------------------------------------- */
/* ------------------------- TESTS ------------------------- */
/* --------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_filename_test() {
        let input = "diff --git a/script.sh b/script.sh\n";
        match parse_filename(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!("script.sh", result.0);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_extended_header_mode_test() {
        let input = "old mode 100644\nnew mode 100755\n";
        match parse_extended_header_mode(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(ExtendedHeader::ChMode(("100644", "100755")), result);
            }
            Err(e) => {
                println!("Error {:?}", e);
            }
        }
    }

    #[test]
    fn parse_extended_header_deleted_test() {
        let input = "deleted file mode 100644\n";
        match parse_extended_header_deleted(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(ExtendedHeader::Deleted, result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_extended_header_new_file_test() {
        let input = "new file mode 100644\n";
        match parse_extended_header_new_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(ExtendedHeader::NewFile, result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_extended_header_copy_file_test() {
        let input = "copy from path/to/file/a\ncopy to path/to/file/b\n";
        match parse_extended_header_copy_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(
                    ExtendedHeader::CopyFile(("path/to/file/a", "path/to/file/b")),
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_extended_header_rename_file_test() {
        let input = "rename from path/to/file/a\nrename to path/to/file/b\n";
        match parse_extended_header_rename_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(
                    ExtendedHeader::RenameFile(("path/to/file/a", "path/to/file/b")),
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_extended_header_similarity_index_test() {
        let input = "similarity index 80%\n";
        match parse_extended_header_similarity_index(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(ExtendedHeader::SimilarityIndex("80%"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_extended_header_dissimilarity_index_test() {
        let input = "dissimilarity index 20%\n";
        match parse_extended_header_dissimilarity_index(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(ExtendedHeader::DissimilarityIndex("20%"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_extended_header_index_test() {
        let input = "index 089fe5f..384ac88 100644\n@@";
        match parse_extended_header_index(&input[..]) {
            Ok((remaining, result)) => {
                assert_eq!("@@", remaining);
                assert_eq!(ExtendedHeader::Index("384ac88"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_raw_file_header_test() {
        let input = r#"diff --git a/file2.txt b/file2.txt
similarity index 80%
dissimilarity index 20%
index 2b2338d..43febe7 100644
--- a/file2.txt
+++ b/file2.txt
"#;
        match parse_raw_file_header(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(
                    RawHeader {
                        filenames: ("file2.txt", "file2.txt"),
                        extended_headers: vec![
                            ExtendedHeader::SimilarityIndex("80%"),
                            ExtendedHeader::DissimilarityIndex("20%"),
                            ExtendedHeader::Index("43febe7"),
                        ]
                    },
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_lines_info_test() {
        let input =
            "--- a/file1.txt\n+++ b/file1.txt\n@@ -1,3 +1,3 @@ first content line of the file\n";
        match parse_lines_info(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!((1, 3, 1, 3), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_line_is_both_test() {
        let input = " This is a line\n";
        match parse_line(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!(RawLine::Both("This is a line"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_line_is_left_test() {
        let input = "-This is a line\n";
        match parse_line(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!(RawLine::Left("This is a line"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_line_is_right_test() {
        let input = "+This is a line\n";
        match parse_line(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!(RawLine::Right("This is a line"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_lines_test() {
        let input = "+This is a line\n this is a both line\n-This is a left line\n Another Both!\n";
        match parse_lines(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!(
                    vec![
                        RawLine::Right("This is a line"),
                        RawLine::Both("this is a both line"),
                        RawLine::Left("This is a left line"),
                        RawLine::Both("Another Both!"),
                    ],
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_raw_file_hunk_test() {
        let input = r#"--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,6 @@
+Add lines on top
+More than one
+So... three
 And lines on top
 very good expanded
 that it must break it
"#;
        match parse_raw_file_hunk(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!(
                    RawHunk {
                        line_info: (1, 3, 1, 6),
                        lines: vec![
                            RawLine::Right("Add lines on top"),
                            RawLine::Right("More than one"),
                            RawLine::Right("So... three"),
                            RawLine::Both("And lines on top"),
                            RawLine::Both("very good expanded"),
                            RawLine::Both("that it must break it"),
                        ]
                    },
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_raw_file_single_hunk_test() {
        let input = r#"diff --git a/file.txt b/file.txt
index c64d930..e475af3 100644
--- a/file.txt
+++ b/file.txt
@@ -1,5 +1,5 @@
 apples
 pears
 strawberries
-bannannass
-peacches
+bananas
+peaches
"#;
        match parse_raw_file(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!(
                    RawFile {
                        header: RawHeader {
                            filenames: ("file.txt", "file.txt"),
                            extended_headers: vec![ExtendedHeader::Index("e475af3")]
                        },
                        hunks: vec![RawHunk {
                            line_info: (1, 5, 1, 5),
                            lines: vec![
                                RawLine::Both("apples"),
                                RawLine::Both("pears"),
                                RawLine::Both("strawberries"),
                                RawLine::Left("bannannass"),
                                RawLine::Left("peacches"),
                                RawLine::Right("bananas"),
                                RawLine::Right("peaches"),
                            ]
                        }]
                    },
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_raw_file_multiple_hunks_test() {
        let input = r#"diff --git a/file.txt b/file.txt
index c5d5782..5014215 100644
--- a/file.txt
+++ b/file.txt
@@ -1,5 +1,4 @@
 apples
-pears
 strawberries
 bananas
 peaches
@@ -14,8 +13,7 @@ tomatoes
 peas
 garlic
 ---
-milk
-cheese
 butter
+cheese
+milk
 whiped cream
-
"#;
        match parse_raw_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(
                    RawFile {
                        header: RawHeader {
                            filenames: ("file.txt", "file.txt"),
                            extended_headers: vec![ExtendedHeader::Index("5014215")]
                        },
                        hunks: vec![
                            RawHunk {
                                line_info: (1, 5, 1, 4),
                                lines: vec![
                                    RawLine::Both("apples"),
                                    RawLine::Left("pears"),
                                    RawLine::Both("strawberries"),
                                    RawLine::Both("bananas"),
                                    RawLine::Both("peaches"),
                                ]
                            },
                            RawHunk {
                                line_info: (14, 8, 13, 7),
                                lines: vec![
                                    RawLine::Both("peas"),
                                    RawLine::Both("garlic"),
                                    RawLine::Both("---"),
                                    RawLine::Left("milk"),
                                    RawLine::Left("cheese"),
                                    RawLine::Both("butter"),
                                    RawLine::Right("cheese"),
                                    RawLine::Right("milk"),
                                    RawLine::Both("whiped cream"),
                                    RawLine::Left("")
                                ]
                            },
                        ]
                    },
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_multiple_raw_files_test() {
        let input = r#"diff --git a/fruits.txt b/fruits.txt
index a4729d6..f3c9161 100644
--- a/fruits.txt
+++ b/fruits.txt
@@ -1,3 +1,3 @@
 apples
-oranges
 bananas
+oranges
diff --git a/spririts.txt b/spririts.txt
index db1afc7..6b65689 100644
--- a/spririts.txt
+++ b/spririts.txt
@@ -1,5 +1,5 @@
-whisky
+gin
 rum
 tekila
 vodka
-gin
+whisky
"#;
        match parse_raw_files_intern(&input[..]) {
            Ok((_remaining, result)) => assert_eq!(
                vec![
                    RawFile {
                        header: RawHeader {
                            filenames: ("fruits.txt", "fruits.txt"),
                            extended_headers: vec![ExtendedHeader::Index("f3c9161")]
                        },
                        hunks: vec![RawHunk {
                            line_info: (1, 3, 1, 3),
                            lines: vec![
                                RawLine::Both("apples"),
                                RawLine::Left("oranges"),
                                RawLine::Both("bananas"),
                                RawLine::Right("oranges"),
                            ]
                        }]
                    },
                    RawFile {
                        header: RawHeader {
                            filenames: ("spririts.txt", "spririts.txt"),
                            extended_headers: vec![ExtendedHeader::Index("6b65689")]
                        },
                        hunks: vec![RawHunk {
                            line_info: (1, 5, 1, 5),
                            lines: vec![
                                RawLine::Left("whisky"),
                                RawLine::Right("gin"),
                                RawLine::Both("rum"),
                                RawLine::Both("tekila"),
                                RawLine::Both("vodka"),
                                RawLine::Left("gin"),
                                RawLine::Right("whisky"),
                            ]
                        }]
                    },
                ],
                result
            ),
            Err(e) => {
                println!("Error: {:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_content_test() {
        let input = r#"diff --git a/list.txt b/list.txt
index 5005045..73ea95f 100644
--- a/list.txt
+++ b/list.txt
@@ -1,4 +1,4 @@
-apples
 oranges
+pears
 pineapples
-kiwis
+kiwi
"#;
        let result = parse_content(&input);
        let expected = File::new(
            MODIFIER::MODIFIED,
            "list.txt".to_string(),
            "73ea95f".to_string(),
            vec![Hunk::new(vec![
                LINE::REM {
                    number: 1,
                    line: "apples".to_string(),
                },
                LINE::NOP {
                    number_left: 2,
                    number_right: 1,
                    line: "oranges".to_string(),
                },
                LINE::ADD {
                    number: 2,
                    line: "pears".to_string(),
                },
                LINE::NOP {
                    number_left: 3,
                    number_right: 3,
                    line: "pineapples".to_string(),
                },
                LINE::REM {
                    number: 4,
                    line: "kiwis".to_string(),
                },
                LINE::ADD {
                    number: 4,
                    line: "kiwi".to_string(),
                },
            ])],
        );
        assert_eq!(vec![expected], result)
    }

    #[test]
    // fn parse_content_2_test() {
    //     let input = r#"diff --git a/list.txt b/list.txt
    // index 5005045..73ea95f 100644
    // --- a/list.txt
    // +++ b/list.txt
    // @@ -1,4 +1,4 @@
    // -apples
    // oranges
    // +pears
    // pineapples
    // -kiwis
    // +kiwi
    // diff --git a/list2.txt b/list2.txt
    // index c5b683b..05f5efa 100644
    // --- a/list2.txt
    // +++ b/list2.txt
    // @@ -1,7 +1,9 @@
    // milk
    // -butter
    // cheese
    // bread
    // -crackers
    // +cracker
    // juice
    // joghurt
    // +wine
    // +beers
    // +water
    // "#
    //     .into();
    //     let result = parse_content(&input);
    //     let expected_file_1 = File::new(
    //         MODIFIER::MODIFIED,
    //         "list.txt".into(),
    //         "73ea95f".into(),
    //         vec![Hunk::new(vec![
    //             LINE::REM((1, "apples".into())),
    //             LINE::NOP((2, 1, "oranges".into())),
    //             LINE::ADD((2, "pears".into())),
    //             LINE::NOP((3, 3, "pineapples".into())),
    //             LINE::REM((4, "kiwis".into())),
    //             LINE::ADD((4, "kiwi".into())),
    //         ])],
    //     );
    //     let expected_file_2 = File::new(
    //         MODIFIER::MODIFIED,
    //         "list2.txt".into(),
    //         "05f5efa".into(),
    //         vec![Hunk::new(vec![
    //             LINE::NOP((1, 1, "milk".into())),
    //             LINE::REM((2, "butter".into())),
    //             LINE::NOP((3, 2, "cheese".into())),
    //             LINE::NOP((4, 3, "bread".into())),
    //             LINE::REM((5, "crackers".into())),
    //             LINE::ADD((4, "cracker".into())),
    //             LINE::NOP((6, 5, "juice".into())),
    //             LINE::NOP((7, 6, "joghurt".into())),
    //             LINE::ADD((7, "wine".into())),
    //             LINE::ADD((8, "beers".into())),
    //             LINE::ADD((9, "water".into())),
    //         ])],
    //     );
    //     assert_eq!(vec![expected_file_1, expected_file_2], result)
    // }
    #[test]
    // fn parse_content_renamed_file_test() {
    //     let input = r#"diff --git a/list.txt b/list_renamed.txt
    // similarity index 100%
    // rename from list.txt
    // rename to list_renamed.txt
    // index 0000000..33e4d8e
    // "#
    //     .into();
    //     let result = parse_content(&input);
    //     let expected_file = File::new(
    //         MODIFIER::RENAMED,
    //         "list.txt".into(),
    //         "00000000".into(),
    //         vec![Hunk::new(vec![
    //             LINE::REM((1, "oranges".into())),
    //             LINE::REM((2, "pears".into())),
    //             LINE::REM((3, "pineapples".into())),
    //             LINE::REM((4, "kiwi".into())),
    //         ])],
    //     );
    //     assert_eq!(vec![expected_file], result)
    // }
    #[test]
    fn parse_content_multiple_files_test() {
        let input = r#"diff --git a/list3.txt b/list3.txt
new file mode 100644
index 0000000..33e4d8e
--- /dev/null
+++ b/list3.txt
@@ -0,0 +1,3 @@
+bananas
+apples
+oranges
diff --git a/list.txt b/list.txt
deleted file mode 100644
index 73ea95f..0000000
--- a/list.txt
+++ /dev/null
@@ -1,4 +0,0 @@
-oranges
-pears
-pineapples
-kiwi
diff --git a/list.txt b/list_renamed.txt
similarity index 100%
rename from list.txt
rename to list_renamed.txt
"#;
        let result = parse_content(&input);
        let expected_file_1 = File::new(
            MODIFIER::ADD,
            "list3.txt".into(),
            "33e4d8e".into(),
            vec![Hunk::new(vec![
                LINE::ADD {
                    number: 1,
                    line: "bananas".into(),
                },
                LINE::ADD {
                    number: 2,
                    line: "apples".into(),
                },
                LINE::ADD {
                    number: 3,
                    line: "oranges".into(),
                },
            ])],
        );
        let expected_file_2 = File::new(
            MODIFIER::DELETE,
            "list.txt".into(),
            "0000000".into(),
            vec![Hunk::new(vec![
                LINE::REM {
                    number: 1,
                    line: "oranges".into(),
                },
                LINE::REM {
                    number: 2,
                    line: "pears".into(),
                },
                LINE::REM {
                    number: 3,
                    line: "pineapples".into(),
                },
                LINE::REM {
                    number: 4,
                    line: "kiwi".into(),
                },
            ])],
        );
        let expected_file_3 = File::new(MODIFIER::RENAMED, "list.txt".into(), "".into(), vec![]);
        assert_eq!(
            vec![expected_file_1, expected_file_2, expected_file_3],
            result
        )
    }
}
