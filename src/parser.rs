//! Preprocess the input to convert it to raw structures

use file::{File, Hunk, LINE, MODIFIER};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum RawLine<'a> {
    Left(&'a str),
    Right(&'a str),
    Both(&'a str),
}

#[derive(Debug, PartialEq)]
struct RawFileHeader<'a> {
    filenames: (&'a str, &'a str),
    modifier: MODIFIER,
    commit_id: &'a str,
}

#[derive(Debug, PartialEq)]
struct RawFileHunk<'a> {
    line_info: (u32, u32, u32, u32),
    lines: Vec<RawLine<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct RawFile<'a> {
    header: RawFileHeader<'a>,
    hunks: Vec<RawFileHunk<'a>>,
}

#[allow(dead_code)]
fn is_space(c: char) -> bool {
    nom::is_space(c as u8)
}
#[allow(dead_code)]
fn is_new_line(c: char) -> bool {
    c == '\n'
}

// "diff --git a/script.sh b/script.sh\n"
named!(parse_filename(&str) -> ( &str, &str ), do_parse!(
        opt!(take_until_and_consume!("diff --git a/")) >>
        filename_a: take_until_and_consume!(" b/") >>
        filename_b: take_until_and_consume!("\n") >>
        ((filename_a, filename_b))
));

// "deleted file mode 100644\n";
named!(parse_delete_file(&str) -> Option<&str>,
        opt!(alt!( tag!("deleted") | tag!("new file") | tag!("rename") ))
);

// "index 089fe5f..384ac88 100644\n";
named!(parse_commit_id(&str) -> &str, do_parse!(
        opt!(take_until_and_consume!("index")) >>
        take_until_and_consume!("..") >>
        commit_id: take_till!(is_space) >>
        take_until!("@@") >>
        (commit_id)
));

named!(parse_raw_file_header(&str) -> RawFileHeader, do_parse!(
        filenames: parse_filename >>
        is_deleted: parse_delete_file >>
        commit_id: parse_commit_id >>
        (RawFileHeader {
            filenames,
            modifier: match is_deleted {
                Some(modifier) => {
                    match modifier {
                        "deleted" => MODIFIER::DELETE,
                        "new file" => MODIFIER::ADD,
                        "rename" => MODIFIER::RENAMED,
                        _ => unreachable!()
                    }
                },
                None => MODIFIER::MODIFIED,
            },
            commit_id
        })
));

named!(parse_u32(&str) -> Result<u32, std::num::ParseIntError>,
    map!(nom::digit, std::str::FromStr::from_str)
);

// "@@ -1,3 +1,3 @@\n";
named!(parse_lines_info(&str) -> (u32, u32, u32, u32), do_parse!(
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

named!(parse_line_both(&str) -> RawLine, do_parse!(
        tag!(" ") >>
        content: take_till!(is_new_line) >>
        (RawLine::Both(content))
));

named!(parse_line_left(&str) -> RawLine, do_parse!(
        tag!("-") >>
        content: take_till!(is_new_line) >>
        (RawLine::Left(content))
));

named!(parse_line_right(&str) -> RawLine, do_parse!(
        tag!("+") >>
        content: take_till!(is_new_line) >>
        (RawLine::Right(content))
));

named!(parse_line(&str) -> RawLine, do_parse!(
        line: alt!(parse_line_both | parse_line_left | parse_line_right) >>
        opt!(alt!(tag!("\n") | eof!())) >>
        (line)
));

named!(parse_lines(&str) -> Vec<RawLine>,
       many0!(complete!(parse_line))
);

named!(parse_raw_file_hunk(&str) -> RawFileHunk, do_parse!(
        line_info: parse_lines_info >>
        lines: parse_lines >>
        (RawFileHunk {
            line_info,
            lines
        })
));

named!(parse_raw_file(&str) -> RawFile, do_parse!(
        header: parse_raw_file_header >>
        hunks: many0!(complete!(parse_raw_file_hunk)) >>
        (RawFile {
            header,
            hunks
        })
));

named!(parse_raw_files_intern(&str) -> Vec<RawFile>,
       many0!(complete!(parse_raw_file))
);

pub fn parse_raw_files<'a>(input: &'a str) -> Result<Vec<RawFile<'a>>, String> {
    match parse_raw_files_intern(input) {
        Ok((_remaining, result)) => {
            if !_remaining.is_empty() {
                Err("Error parsing file.".into())
            } else {
                Ok(result)
            }
        }
        Err(e) => Err(format!("Error parsing file. {}", e)),
    }
}

pub fn parse_content(input: &String) -> Vec<File> {
    let raw_files: Vec<RawFile> = parse_raw_files(input).unwrap();

    let mut parsed_files: Vec<File> = Vec::new();

    for raw_file in raw_files {
        let modifier = raw_file.header.modifier;
        let filename: String = raw_file.header.filenames.0.into();
        let commit_id: String = raw_file.header.commit_id.into();

        let mut hunks: Vec<Hunk> = Vec::new();
        for hunk in &raw_file.hunks {
            let mut lines: Vec<LINE> = Vec::new();
            let mut line_nr_left = hunk.line_info.0;
            let mut line_nr_right = hunk.line_info.2;

            for line in &hunk.lines {
                match line {
                    RawLine::Left(content) => {
                        lines.push(LINE::REM((line_nr_left as usize, String::from(*content))));
                        line_nr_left += 1;
                    }
                    RawLine::Right(content) => {
                        lines.push(LINE::ADD((line_nr_right as usize, String::from(*content))));
                        line_nr_right += 1;
                    }
                    RawLine::Both(content) => {
                        lines.push(LINE::NOP((
                            line_nr_left as usize,
                            line_nr_right as usize,
                            String::from(*content),
                        )));
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
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_deleted_file_test() {
        let input = "deleted file mode 100644\n";
        match parse_delete_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(Some("deleted"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_not_deleted_file_test() {
        let input = "file mode 100644\n";
        match parse_delete_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(None, result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_commit_id_test() {
        let input = "index 089fe5f..384ac88 100644\n...@@";
        match parse_commit_id(&input[..]) {
            Ok((remaining, result)) => {
                assert_eq!("@@", remaining);
                assert_eq!("384ac88", result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_raw_file_header_test() {
        let input = r#"diff --git a/file2_renamed.txt b/file2_renamed.txt
index 2b2338d..43febe7 100644
--- a/file2_renamed.txt
+++ b/file2_renamed.txt
@@"#;
        match parse_raw_file_header(&input[..]) {
            Ok((remaining, result)) => {
                assert_eq!("@@", remaining);
                assert_eq!(
                    RawFileHeader {
                        filenames: ("file2_renamed.txt", "file2_renamed.txt",),
                        modifier: MODIFIER::MODIFIED,
                        commit_id: "43febe7",
                    },
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_lines_info_test() {
        let input = "@@ -1,3 +1,3 @@ first content line of the file\n";
        match parse_lines_info(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!((1, 3, 1, 3), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_line_both_test() {
        let input = " This is a line\n";
        match parse_line_both(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(RawLine::Both("This is a line"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_line_left_test() {
        let input = "-This is a line\n";
        match parse_line_left(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(RawLine::Left("This is a line"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_line_right_test() {
        let input = "+This is a line\n";
        match parse_line_right(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(RawLine::Right("This is a line"), result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
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
                assert!(false)
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
                assert!(false)
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
                assert!(false)
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
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_raw_file_hunk_test() {
        let input = r#"@@ -1,3 +1,6 @@
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
                    RawFileHunk {
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
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_raw_file_single_hunk_test() {
        let input = r#"diff --git a/src/main.rs b/src/main.rs
index 82ef95f..1f77505 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -47,9 +47,9 @@ fn main() {
     // }
 a
     // let files: Vec<file::File> = parse_content(&lines);
-    let files: Vec<file::File> = parse_content(buffer.into_bytes());
+    // let files: Vec<file::File> = parse_content(buffer.into_bytes());
 a
-    printer::print(&files);
+    // printer::print(&files);
 }
 a
 // Test cases
"#;
        match parse_raw_file(&input[..]) {
            Ok((remaining, result)) => {
                assert!(remaining.is_empty());
                assert_eq!(
                RawFile {
                    header: RawFileHeader {
                        filenames: ("src/main.rs", "src/main.rs"),
                        modifier: MODIFIER::MODIFIED,
                        commit_id: "1f77505"
                    },
                    hunks: vec![RawFileHunk {
                        line_info: (47, 9, 47, 9),
                        lines: vec![
                            RawLine::Both("    // }"),
                            RawLine::Both("a"),
                            RawLine::Both("    // let files: Vec<file::File> = parse_content(&lines);"),
                            RawLine::Left("    let files: Vec<file::File> = parse_content(buffer.into_bytes());"),
                            RawLine::Right("    // let files: Vec<file::File> = parse_content(buffer.into_bytes());"),
                            RawLine::Both("a"),
                            RawLine::Left("    printer::print(&files);"),
                            RawLine::Right("    // printer::print(&files);"),
                            RawLine::Both("}"),
                            RawLine::Both("a"),
                            RawLine::Both("// Test cases"),
                        ]
                    }]
                },
                result
            );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_raw_file_multiple_hunks_test() {
        let input = r#"diff --git a/file2.txt b/file2.txt
index 772563d..01d1a6e 100644
--- a/file2_renamed.txt
+++ b/file2_renamed.txt
@@ -1,3 +1,7 @@
+And lines on top
+very good expanded
+that it must break it
+in two parts...
 This is file 2
 Line betwern
 Second line
@@ -5,4 +9,7 @@ line at the end
 Line
 stays
 here
+And even more lines
+
 Adding more lines of ...
+And more and more
"#;
        match parse_raw_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(
                    RawFile {
                        header: RawFileHeader {
                            filenames: ("file2.txt", "file2.txt"),
                            modifier: MODIFIER::MODIFIED,
                            commit_id: "01d1a6e"
                        },
                        hunks: vec![
                            RawFileHunk {
                                line_info: (1, 3, 1, 7),
                                lines: vec![
                                    RawLine::Right("And lines on top"),
                                    RawLine::Right("very good expanded"),
                                    RawLine::Right("that it must break it"),
                                    RawLine::Right("in two parts..."),
                                    RawLine::Both("This is file 2"),
                                    RawLine::Both("Line betwern"),
                                    RawLine::Both("Second line"),
                                ]
                            },
                            RawFileHunk {
                                line_info: (5, 4, 9, 7),
                                lines: vec![
                                    RawLine::Both("Line"),
                                    RawLine::Both("stays"),
                                    RawLine::Both("here"),
                                    RawLine::Right("And even more lines"),
                                    RawLine::Right(""),
                                    RawLine::Both("Adding more lines of ..."),
                                    RawLine::Right("And more and more"),
                                ]
                            },
                        ]
                    },
                    result
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_multiple_raw_files_test() {
        let input = r#"diff --git a/file1.txt b/file1.txt
index 534cc51..0ee5d0d 100644
--- a/file1.txt
+++ b/file1.txt
@@ -1,1 +1,3 @@
-Remove line and add another
+Remove line and add anothers
+Add more lines
+And change the first
diff --git a/file2_renamed.txt b/file2_renamed.txt
index 35dee2c..a66e579 100644
--- a/file2_renamed.txt
+++ b/file2_renamed.txt
@@ -1,4 +1,4 @@
 This is file 2
-Line between
+Line betwern
 Second line
 line at the end
"#;
        match parse_raw_files_intern(&input[..]) {
            Ok((_remaining, result)) => assert_eq!(
                vec![
                    RawFile {
                        header: RawFileHeader {
                            filenames: ("file1.txt", "file1.txt"),
                            modifier: MODIFIER::MODIFIED,
                            commit_id: "0ee5d0d"
                        },
                        hunks: vec![RawFileHunk {
                            line_info: (1, 1, 1, 3),
                            lines: vec![
                                RawLine::Left("Remove line and add another"),
                                RawLine::Right("Remove line and add anothers"),
                                RawLine::Right("Add more lines"),
                                RawLine::Right("And change the first"),
                            ]
                        }]
                    },
                    RawFile {
                        header: RawFileHeader {
                            filenames: ("file2_renamed.txt", "file2_renamed.txt"),
                            modifier: MODIFIER::MODIFIED,
                            commit_id: "a66e579"
                        },
                        hunks: vec![RawFileHunk {
                            line_info: (1, 4, 1, 4),
                            lines: vec![
                                RawLine::Both("This is file 2"),
                                RawLine::Left("Line between"),
                                RawLine::Right("Line betwern"),
                                RawLine::Both("Second line"),
                                RawLine::Both("line at the end"),
                            ]
                        }]
                    },
                ],
                result
            ),
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_content_1_test() {
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
"#
        .into();
        let result = parse_content(&input);
        let expected = File::new(
            MODIFIER::MODIFIED,
            "list.txt".to_string(),
            "73ea95f".to_string(),
            vec![Hunk::new(vec![
                LINE::REM((1, "apples".to_string())),
                LINE::NOP((2, 1, "oranges".to_string())),
                LINE::ADD((2, "pears".to_string())),
                LINE::NOP((3, 3, "pineapples".to_string())),
                LINE::REM((4, "kiwis".to_string())),
                LINE::ADD((4, "kiwi".to_string())),
            ])],
        );
        assert_eq!(vec![expected], result)
    }

    #[test]
    fn parse_content_2_test() {
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
diff --git a/list2.txt b/list2.txt
index c5b683b..05f5efa 100644
--- a/list2.txt
+++ b/list2.txt
@@ -1,7 +1,9 @@
 milk
-butter
 cheese
 bread
-crackers
+cracker
 juice
 joghurt
+wine
+beers
+water
"#
        .into();
        let result = parse_content(&input);
        let expected_file_1 = File::new(
            MODIFIER::MODIFIED,
            "list.txt".into(),
            "73ea95f".into(),
            vec![Hunk::new(vec![
                LINE::REM((1, "apples".into())),
                LINE::NOP((2, 1, "oranges".into())),
                LINE::ADD((2, "pears".into())),
                LINE::NOP((3, 3, "pineapples".into())),
                LINE::REM((4, "kiwis".into())),
                LINE::ADD((4, "kiwi".into())),
            ])],
        );
        let expected_file_2 = File::new(
            MODIFIER::MODIFIED,
            "list2.txt".into(),
            "05f5efa".into(),
            vec![Hunk::new(vec![
                LINE::NOP((1, 1, "milk".into())),
                LINE::REM((2, "butter".into())),
                LINE::NOP((3, 2, "cheese".into())),
                LINE::NOP((4, 3, "bread".into())),
                LINE::REM((5, "crackers".into())),
                LINE::ADD((4, "cracker".into())),
                LINE::NOP((6, 5, "juice".into())),
                LINE::NOP((7, 6, "joghurt".into())),
                LINE::ADD((7, "wine".into())),
                LINE::ADD((8, "beers".into())),
                LINE::ADD((9, "water".into())),
            ])],
        );
        assert_eq!(vec![expected_file_1, expected_file_2], result)
    }
}
