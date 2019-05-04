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
    extended_headers: Vec<ExtendedHeader<'a>>,
struct RawHunk<'a> {
    header: RawHeader<'a>,
    hunks: Vec<RawHunk<'a>>,
#[allow(dead_code)]
fn is_whitespace(c: char) -> bool {
    is_space(c) || is_new_line(c)
}
named!(parse_filename(&str) -> (&str, &str), do_parse!(
named!(parse_extended_header_mode(&str) -> ExtendedHeader, do_parse!(
        tag!("old mode ") >>
        old_mode: take_until_and_consume!("\n") >>
        tag!("new mode ") >>
        new_mode: take_until_and_consume!("\n") >>
        (ExtendedHeader::ChMode((old_mode, new_mode)))
));

named!(parse_extended_header_deleted(&str) -> ExtendedHeader, do_parse!(
        tag!("deleted") >> take_until_and_consume!("\n") >> (ExtendedHeader::Deleted)
));

named!(parse_extended_header_new_file(&str) -> ExtendedHeader, do_parse!(
        tag!("new file") >> take_until_and_consume!("\n") >> (ExtendedHeader::NewFile)
));

named!(parse_extended_header_copy_file(&str) -> ExtendedHeader, do_parse!(
        tag!("copy from ") >>
        from_path: take_until_and_consume!("\n") >>
        tag!("copy to ") >>
        to_path: take_until_and_consume!("\n") >>
        (ExtendedHeader::CopyFile((from_path, to_path)))
));
named!(parse_extended_header_rename_file(&str) -> ExtendedHeader, do_parse!(
        tag!("rename from ") >>
        from_path: take_until_and_consume!("\n") >>
        tag!("rename to ") >>
        to_path: take_until_and_consume!("\n") >>
        (ExtendedHeader::RenameFile((from_path, to_path)))
));

named!(parse_extended_header_similarity_index(&str) -> ExtendedHeader, do_parse!(
        tag!("similarity index ") >>
        index: take_until_and_consume!("\n") >>
        (ExtendedHeader::SimilarityIndex(index))
));

named!(parse_extended_header_dissimilarity_index(&str) -> ExtendedHeader, do_parse!(
        tag!("dissimilarity index ") >>
        index: take_until_and_consume!("\n") >>
        (ExtendedHeader::DissimilarityIndex(index))
));

named!(parse_extended_header_index(&str) -> ExtendedHeader, do_parse!(
        tag!("index") >>
        index: take_till!(is_whitespace) >>
        take_until_and_consume!("\n") >>
        (ExtendedHeader::Index(index))
named!(parse_extended_header(&str) -> ExtendedHeader, do_parse!(
        extended_header: alt!(
            parse_extended_header_mode | parse_extended_header_deleted |
            parse_extended_header_new_file | parse_extended_header_copy_file |
            parse_extended_header_rename_file | parse_extended_header_similarity_index |
            parse_extended_header_dissimilarity_index | parse_extended_header_index ) >>
        (extended_header)));

named!(parse_raw_file_header(&str) -> RawHeader, do_parse!(
        extended_headers: many0!(complete!(parse_extended_header)) >>
        (RawHeader {
            extended_headers
named!(parse_file_names_after_extended_header(&str) -> (), do_parse!(
        tag!("--- ") >> take_until_and_consume!("\n") >>
        tag!("+++ ") >> take_until_and_consume!("\n") >>
        ()
));

        opt!(parse_file_names_after_extended_header) >>
named!(parse_raw_file_hunk(&str) -> RawHunk, do_parse!(
        (RawHunk {
        header: complete!(parse_raw_file_header) >>
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(format!(
                    "Error parsing file. Remaining is not empty. Remaining: {:?}",
                    remaining
                ))
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
        match parse_extended_header_deleted(&input[..]) {
                assert_eq!(ExtendedHeader::Deleted, result);
    fn parse_extended_header_new_file_test() {
        let input = "new file mode 100644\n";
        match parse_extended_header_new_file(&input[..]) {
                assert_eq!(ExtendedHeader::NewFile, result);
    fn parse_extended_header_copy_file_test() {
        let input = "copy from path/to/file/a\ncopy to path/to/file/b\n";
        match parse_extended_header_copy_file(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(
                    ExtendedHeader::CopyFile(("path/to/file/a", "path/to/file/b")),
                    result
                );
    fn parse_extended_header_rename_file_test() {
        let input = "rename from path/to/file/a\nrename to path/to/file/b\n";
        match parse_extended_header_rename_file(&input[..]) {
            Ok((_remaining, result)) => {
                    ExtendedHeader::RenameFile(("path/to/file/a", "path/to/file/b")),
    fn parse_extended_header_similarity_index_test() {
        let input = "similarity index 80%\n";
        match parse_extended_header_similarity_index(&input[..]) {
            Ok((_remaining, result)) => {
                assert_eq!(ExtendedHeader::SimilarityIndex("80%"), result);
    fn parse_extended_header_dissimilarity_index_test() {
        let input = "dissimilarity index 20%\n";
        match parse_extended_header_dissimilarity_index(&input[..]) {
                assert_eq!(ExtendedHeader::DissimilarityIndex("20%"), result);
    fn parse_extended_header_index_test() {
        let input = "index 089fe5f..384ac88 100644\n@@";
        match parse_extended_header_index(&input[..]) {
            Ok((remaining, result)) => {
                assert_eq!("@@", remaining);
                assert_eq!(ExtendedHeader::Index("384ac88"), result);
    fn parse_raw_file_header_test() {
        let input = r#"diff --git a/file2.txt b/file2.txt
similarity index 80%
dissimilarity index 20%
index 2b2338d..43febe7 100644
--- a/file2.txt
+++ b/file2.txt
"#;
        match parse_raw_file_header(&input[..]) {
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
                assert!(false)
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
        let input = r#"--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,6 @@
                    RawHunk {
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
                    result
                );
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
                        header: RawHeader {
                            filenames: ("file.txt", "file.txt"),
                            extended_headers: vec![ExtendedHeader::Index("5014215")]
                            RawHunk {
                                line_info: (1, 5, 1, 4),
                                    RawLine::Both("apples"),
                                    RawLine::Left("pears"),
                                    RawLine::Both("strawberries"),
                                    RawLine::Both("bananas"),
                                    RawLine::Both("peaches"),
                            RawHunk {
                                line_info: (14, 8, 13, 7),
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
                        header: RawHeader {
                            filenames: ("fruits.txt", "fruits.txt"),
                            extended_headers: vec![ExtendedHeader::Index("f3c9161")]
                        hunks: vec![RawHunk {
                            line_info: (1, 3, 1, 3),
                                RawLine::Both("apples"),
                                RawLine::Left("oranges"),
                                RawLine::Both("bananas"),
                                RawLine::Right("oranges"),
                        header: RawHeader {
                            filenames: ("spririts.txt", "spririts.txt"),
                            extended_headers: vec![ExtendedHeader::Index("6b65689")]
                        hunks: vec![RawHunk {
                            line_info: (1, 5, 1, 5),
                                RawLine::Left("whisky"),
                                RawLine::Right("gin"),
                                RawLine::Both("rum"),
                                RawLine::Both("tekila"),
                                RawLine::Both("vodka"),
                                RawLine::Left("gin"),
                                RawLine::Right("whisky"),
    fn parse_content_test() {
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
            MODIFIER::ADD,
            "list3.txt".into(),
            "33e4d8e".into(),
                LINE::ADD((1, "bananas".into())),
                LINE::ADD((2, "apples".into())),
                LINE::ADD((3, "oranges".into())),
            MODIFIER::DELETE,
            "list.txt".into(),
            "0000000".into(),
                LINE::REM((1, "oranges".into())),
                LINE::REM((2, "pears".into())),
                LINE::REM((3, "pineapples".into())),
                LINE::REM((4, "kiwi".into())),
        let expected_file_3 = File::new(MODIFIER::RENAMED, "list.txt".into(), "".into(), vec![]);
        assert_eq!(
            vec![expected_file_1, expected_file_2, expected_file_3],
            result
        )