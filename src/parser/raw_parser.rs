//! Preprocess the input to convert it to raw structures

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum LINE<'a> {
    Left(&'a str),
    Right(&'a str),
    Both(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct RawFileHeader<'a> {
    filenames: (&'a str, &'a str),
    is_deleted: bool,
    commit_id: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct RawFileHunk<'a> {
    line_info: (u32, u32, u32, u32),
    lines: Vec<LINE<'a>>,
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
        opt!(tag!("deleted"))
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
            is_deleted: match is_deleted {
                Some(_) => true,
                None => false,
            },
            commit_id
        })
));

named!(parse_u32(&str) -> Result<u32, std::num::ParseIntError>,
    map!(nom::digit, std::str::FromStr::from_str)
);

// "@@ -1,3 +1,3 @@\n";
named!(parse_lines_info(&str) -> (u32, u32, u32, u32), do_parse!(
        // opt!(take_until_and_consume!("@@ -")) >>
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

named!(parse_line_both(&str) -> LINE, do_parse!(
        tag!(" ") >>
        content: take_till!(is_new_line) >>
        (LINE::Both(content))
));

named!(parse_line_left(&str) -> LINE, do_parse!(
        tag!("-") >>
        content: take_till!(is_new_line) >>
        (LINE::Left(content))
));

named!(parse_line_right(&str) -> LINE, do_parse!(
        tag!("+") >>
        content: take_till!(is_new_line) >>
        (LINE::Right(content))
));

named!(parse_line(&str) -> LINE, do_parse!(
        line: alt!(parse_line_both | parse_line_left | parse_line_right) >>
        opt!(alt!(tag!("\n") | eof!())) >>
        (line)
));

named!(parse_lines(&str) -> Vec<LINE>,
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

pub fn parse_raw_files<'a>(input: &str) -> Vec<RawFile<'a>> {
    vec![]
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
                        is_deleted: false,
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
                assert_eq!(LINE::Both("This is a line"), result);
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
                assert_eq!(LINE::Left("This is a line"), result);
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
                assert_eq!(LINE::Right("This is a line"), result);
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
                assert_eq!(LINE::Both("This is a line"), result);
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
                assert_eq!(LINE::Left("This is a line"), result);
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
                assert_eq!(LINE::Right("This is a line"), result);
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
                        LINE::Right("This is a line"),
                        LINE::Both("this is a both line"),
                        LINE::Left("This is a left line"),
                        LINE::Both("Another Both!"),
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
                            LINE::Right("Add lines on top"),
                            LINE::Right("More than one"),
                            LINE::Right("So... three"),
                            LINE::Both("And lines on top"),
                            LINE::Both("very good expanded"),
                            LINE::Both("that it must break it"),
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
                        is_deleted: false,
                        commit_id: "1f77505"
                    },
                    hunks: vec![RawFileHunk {
                        line_info: (47, 9, 47, 9),
                        lines: vec![
                            LINE::Both("    // }"),
                            LINE::Both("a"),
                            LINE::Both("    // let files: Vec<file::File> = parse_content(&lines);"),
                            LINE::Left("    let files: Vec<file::File> = parse_content(buffer.into_bytes());"),
                            LINE::Right("    // let files: Vec<file::File> = parse_content(buffer.into_bytes());"),
                            LINE::Both("a"),
                            LINE::Left("    printer::print(&files);"),
                            LINE::Right("    // printer::print(&files);"),
                            LINE::Both("}"),
                            LINE::Both("a"),
                            LINE::Both("// Test cases"),
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
                            is_deleted: false,
                            commit_id: "01d1a6e"
                        },
                        hunks: vec![
                            RawFileHunk {
                                line_info: (1, 3, 1, 7),
                                lines: vec![
                                    LINE::Right("And lines on top"),
                                    LINE::Right("very good expanded"),
                                    LINE::Right("that it must break it"),
                                    LINE::Right("in two parts..."),
                                    LINE::Both("This is file 2"),
                                    LINE::Both("Line betwern"),
                                    LINE::Both("Second line"),
                                ]
                            },
                            RawFileHunk {
                                line_info: (5, 4, 9, 7),
                                lines: vec![
                                    LINE::Both("Line"),
                                    LINE::Both("stays"),
                                    LINE::Both("here"),
                                    LINE::Right("And even more lines"),
                                    LINE::Right(""),
                                    LINE::Both("Adding more lines of ..."),
                                    LINE::Right("And more and more"),
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
                            is_deleted: false,
                            commit_id: "0ee5d0d"
                        },
                        hunks: vec![RawFileHunk {
                            line_info: (1, 1, 1, 3),
                            lines: vec![
                                LINE::Left("Remove line and add another"),
                                LINE::Right("Remove line and add anothers"),
                                LINE::Right("Add more lines"),
                                LINE::Right("And change the first"),
                            ]
                        }]
                    },
                    RawFile {
                        header: RawFileHeader {
                            filenames: ("file2_renamed.txt", "file2_renamed.txt"),
                            is_deleted: false,
                            commit_id: "a66e579"
                        },
                        hunks: vec![RawFileHunk {
                            line_info: (1, 4, 1, 4),
                            lines: vec![
                                LINE::Both("This is file 2"),
                                LINE::Left("Line between"),
                                LINE::Right("Line betwern"),
                                LINE::Both("Second line"),
                                LINE::Both("line at the end"),
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
}
