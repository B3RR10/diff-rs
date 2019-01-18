use super::*;

/// Gets an sample file struct and prints it
/// TODO: Add asserts and test for two column option
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

/// Parse a diff file and test to print it
/// TODO: Add asserts and test with parsing errors.
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
