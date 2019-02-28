//! The parser analyses the diff content and return the file(s) to the printer
//! The modified lines get the same line numbers in the hunk content.

// use file::{File, Hunk, Line, LINEMOD, MODIFIER};
use file::File;

#[allow(dead_code)]
fn is_space(c: char) -> bool {
    c == ' '
}
#[allow(dead_code)]
fn is_new_line(c: char) -> bool {
    c == '\n'
}
#[allow(dead_code)]
fn is_coma(c: char) -> bool {
    c == ','
}

// "diff --git a/script.sh b/script.sh\n"
named!(parse_filename(&str) -> ( String, String ), do_parse!(
        tag!("diff --git a/") >>
        filename_a: take_till!(is_space) >>
        tag!(" b/") >>
        filename_b: take_till!(is_new_line) >>
        tag!("\n") >>
        ((filename_a.to_string(), filename_b.to_string()))
));

named!(parse_delete_file(&str) -> Option<&str>, do_parse!(
        del_tag: opt!(tag!("deleted")) >>
        take_till!(is_new_line) >>
        tag!("\n") >>
        (del_tag)
));

named!(parse_commit_id(&str) -> String, do_parse!(
        tag!("index ") >>
        take_till!(|c: char| c == '.') >>
        tag!("..") >>
        commit_id: take_till!(is_space) >>
        take_till!(is_new_line) >>
        tag!("\n") >>
        (commit_id.to_string())
));

named!(parse_u32(&str) -> Result<u32, std::num::ParseIntError>,
    map!(nom::digit, std::str::FromStr::from_str)
);

// let input = "@@ -1,3 +1,3 @@\n";
named!(parse_lines_info(&str) -> (u32, u32, u32), do_parse!(
        tag!("@@ -") >>
        start_line: parse_u32 >>
        tag!(",") >>
        lines_left: parse_u32 >>
        take_till!(is_coma) >>
        tag!(",") >>
        lines_right: parse_u32 >>
        take_till!(is_new_line) >>
        tag!("\n") >>
        (start_line.unwrap(), lines_left.unwrap(), lines_right.unwrap())
));

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum LINE<'a> {
    Left(&'a str),
    Right(&'a str),
    Both(&'a str),
}

named!(parse_line_both(&str) -> LINE, do_parse!(
        tag!(" ") >>
        content: take_till!(is_new_line) >>
        tag!("\n") >>
        (LINE::Both(content))
));

named!(parse_line_left(&str) -> LINE, do_parse!(
        tag!("-") >>
        content: take_till!(is_new_line) >>
        tag!("\n") >>
        (LINE::Left(content))
));

named!(parse_line_right(&str) -> LINE, do_parse!(
        tag!("+") >>
        content: take_till!(is_new_line) >>
        tag!("\n") >>
        (LINE::Right(content))
));

pub fn parse_content(_input: &String) -> Vec<File> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_filename_test() {
        let input = "diff --git a/script.sh b/script.sh\n";
        if let Ok((remaining, result)) = parse_filename(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!("script.sh".to_string(), result.0);
        } else {
            assert!(false)
        }
    }

    #[test]
    pub fn parse_commit_id_test() {
        let input = "index 089fe5f..384ac88 100644\n";
        if let Ok((remaining, result)) = parse_commit_id(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!("384ac88".to_string(), result);
        } else {
            assert!(false)
        }
    }

    #[test]
    pub fn parse_deleted_file_test() {
        let input = "deleted file mode 100644\n";
        if let Ok((remaining, result)) = parse_delete_file(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!(Some("deleted"), result);
        } else {
            assert!(false)
        }
    }

    #[test]
    pub fn parse_not_deleted_file_test() {
        let input = "file mode 100644\n";
        if let Ok((remaining, result)) = parse_delete_file(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!(None, result);
        } else {
            assert!(false)
        }
    }

    #[test]
    pub fn parse_lines_info_test() {
        let input = "@@ -1,3 +1,3 @@\n";
        if let Ok((remaining, result)) = parse_lines_info(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!((1, 3, 3), result);
        } else {
            assert!(false)
        }
    }

    #[test]
    pub fn parse_line_both_test() {
        let input = " This is a line\n";
        if let Ok((remaining, result)) = parse_line_both(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!(LINE::Both("This is a line"), result);
        } else {
            assert!(false)
        }
    }

    #[test]
    pub fn parse_line_left_test() {
        let input = "-This is a line\n";
        if let Ok((remaining, result)) = parse_line_left(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!(LINE::Left("This is a line"), result);
        } else {
            assert!(false)
        }
    }

    #[test]
    pub fn parse_line_right_test() {
        let input = "+This is a line\n";
        if let Ok((remaining, result)) = parse_line_right(&input[..]) {
            assert!(remaining.is_empty());
            assert_eq!(LINE::Right("This is a line"), result);
        } else {
            assert!(false)
        }
    }
}
