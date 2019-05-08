//! For printing the diff content in a modern output style to the terminal,
//! this modul prints the file(s) objects from the parser with code
//! highlighting and a colourful diff

extern crate ansi_term;
extern crate term_size;

use self::ansi_term::{Colour, Style};
use file::{File, LINE, MODIFIER};

// file border colour
const FIXED_COLOUR: u8 = 244;

// char definitions
// for border, modifier and the outline painting
const LINE: char = '─';
const LINE_ANCHOR_UP: char = '┬';
const LINE_ANCHOR_MIDDLE: char = '┼';
const LINE_ANCHOR_DOWN: char = '┴';
const LINENUMBER_SEPERATOR: char = '│';
const LINE_CUT1: char = '⸝';
const LINE_CUT2: char = '⸜';
const LINE_CUT3: char = '⸍';
const LINE_CUT4: char = '⸌';
const MODIFIER_ADD: char = 'A';
const MODIFIER_MODIFIED: char = 'M';
const MODIFIER_DELETE: char = 'D';

/// Main print method for printing the file content and the styling
///
/// # Arguments
///
/// * `files` - files that will be printed
///
pub fn print(files: &Vec<File>, _columnview: Option<&str>) -> String {
    let mut printable_output: String = String::new();
    let terminal_size = term_size::dimensions();
    let term_width = terminal_size.unwrap_or((0, 0)).0;

    // for every file in the diff
    files.iter().for_each(|file| {
        // linenumber width
        let max_line_number = file.get_max_line_number_size();
        let ln_width = max_line_number.to_string().chars().count() + 3;

        // filename
        printable_output.push_str(&get_horizontal_line(&term_width, &ln_width, LINE_ANCHOR_UP));
        printable_output.push_str(&get_filename(
            &file.modifier,
            &file.filename,
            &file.commit_id,
            &ln_width,
        ));
        printable_output.push_str(&get_horizontal_line(
            &term_width,
            &ln_width,
            LINE_ANCHOR_MIDDLE,
        ));

        // hunks
        for i in 0..file.hunks.len() {
            for line in &file.hunks[i].content {
                printable_output.push_str(&get_line_content(&ln_width, &line));
            }
            if file.hunks.len() > 1 && file.hunks.len() - 1 != i {
                printable_output.push_str(&get_cut(&term_width));
            }
        }

        printable_output.push_str(&get_horizontal_line(
            &term_width,
            &ln_width,
            LINE_ANCHOR_DOWN,
        ));
    });

    printable_output
}

/// Returns a horizontal line at the beginning, after the filename and at the
/// end of a file.
///
/// # Arguments
///
/// * `width` - the terminal width for line length
/// * `ln_width` - the width of the linenumbers column
/// * `indent_char` - the char to print at the indent for the vertical column
/// line
///
fn get_horizontal_line(width: &usize, ln_width: &usize, indent_char: char) -> String {
    let mut line = String::new();
    for i in 1..*width {
        if i == *ln_width {
            line.push(indent_char);
        }
        line.push(LINE);
    }
    line.push_str("\n");
    Colour::Fixed(FIXED_COLOUR).paint(line).to_string()
}

/// Returns a outline after every hunk in a file to show the cut in a file.
///
/// # Arguments
///
/// * `width` - the terminal width for line length
///
fn get_cut(width: &usize) -> String {
    let mut output = String::new();
    // down cut
    for _ in (1..*width).step_by(2) {
        output.push_str(&format!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT1.to_string())
        ));
        output.push_str(&format!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT2.to_string())
        ));
    }
    output.push_str("\n");

    // up cut
    for _ in (1..*width).step_by(2) {
        output.push_str(&format!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT3.to_string())
        ));
        output.push_str(&format!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT4.to_string())
        ));
    }
    output.push_str("\n");
    output
}

/// Returns the filename in the header row of a file
///
/// # Arguments
///
/// * `modifier` - the git modifier (add, delete, ...)
/// * `filename` - the filename
/// * `commit_id` - commit id of the file
/// * `ln_width` - linenumber column width for indent
///
fn get_filename(modifier: &MODIFIER, filename: &str, commit_id: &str, ln_width: &usize) -> String {
    let mut output = String::new();
    let modifier_symbol = match modifier {
        MODIFIER::ADD => Colour::Green.bold().paint(MODIFIER_ADD.to_string()),
        MODIFIER::MODIFIED => Colour::Yellow.bold().paint(MODIFIER_MODIFIED.to_string()),
        MODIFIER::RENAMED => Colour::Purple.bold().paint(MODIFIER_MODIFIED.to_string()),
        MODIFIER::DELETE => Colour::Red.bold().paint(MODIFIER_DELETE.to_string()),
    };

    for _ in 1..*ln_width {
        output.push_str(" ");
    }
    output.push_str(&format!(
        "{} {} {} {}{}\n",
        Colour::Fixed(FIXED_COLOUR).paint("│"),
        modifier_symbol,
        Style::new().bold().paint(filename),
        Colour::Blue.bold().paint("@"),
        Colour::Blue.paint(commit_id),
    ));

    output
}

fn get_line_number(ln_width: &usize, line_number: &usize) -> String {
    let mut output = String::new();
    for i in 1..*ln_width {
        if i + line_number.to_string().chars().count() + 1 == *ln_width {
            output.push_str(&format!(
                "{} ",
                Colour::Fixed(FIXED_COLOUR).paint(line_number.to_string())
            ));
            break;
        } else {
            output.push_str(" ");
        }
    }
    output.push_str(&format!(
        "{}",
        Colour::Fixed(FIXED_COLOUR).paint(LINENUMBER_SEPERATOR.to_string())
    ));

    output
}

/// Returns the line content with the different colours for the diff
///
/// # Arguments
///
/// * `ln_width` - linenumber column width for indent
/// * `line` - the line object with their modifiers and content
///
fn get_line_content(ln_width: &usize, line: &LINE) -> String {
    let mut output = String::new();
    match line {
        LINE::ADD { number, line } => {
            output.push_str(&get_line_number(&ln_width, &number));
            output.push_str(&format!(
                "{}\n",
                Colour::Green.paint(format!("+{}", line.to_string()))
            ))
        }
        LINE::REM { number, line } => {
            output.push_str(&get_line_number(&ln_width, &number));
            output.push_str(&format!(
                "{}\n",
                Colour::Red.paint(format!("-{}", line.to_string()))
            ))
        }
        LINE::NOP {
            number_left: _,
            number_right,
            line,
        } => {
            output.push_str(&get_line_number(&ln_width, &number_right));
            output.push_str(&format!(
                "{}\n",
                Colour::White.paint(format!(" {}", line.to_string()))
            ))
        }
    }

    output
}
/* --------------------------------------------------------- */
/* ------------------------- TESTS ------------------------- */
/* --------------------------------------------------------- */
#[cfg(test)]
mod tests {

    use super::super::file::Hunk;
    use super::*;
    #[test]
    fn print_file_test() {
        let term_width = term_size::dimensions().unwrap().0;
        let expected_output = format!("{}   \u{1b}[38;5;244m│\u{1b}[0m \u{1b}[1;33mM\u{1b}[0m \u{1b}[1mfilename.rs\u{1b}[0m \u{1b}[1;34m@\u{1b}[0m\u{1b}[34m23jh23lkl\u{1b}[0m\n{} \u{1b}[38;5;244m4\u{1b}[0m \u{1b}[38;5;244m│\u{1b}[0m\u{1b}[32m+added line...\u{1b}[0m\n \u{1b}[38;5;244m6\u{1b}[0m \u{1b}[38;5;244m│\u{1b}[0m\u{1b}[37m line...\u{1b}[0m\n \u{1b}[38;5;244m9\u{1b}[0m \u{1b}[38;5;244m│\u{1b}[0m\u{1b}[31m-removed line...\u{1b}[0m\n{}", get_horizontal_line(&term_width, &4, LINE_ANCHOR_UP), get_horizontal_line(&term_width, &4, LINE_ANCHOR_MIDDLE), get_horizontal_line(&term_width, &4, LINE_ANCHOR_DOWN));
        let file: File = File::new(
            MODIFIER::MODIFIED,
            "filename.rs".into(),
            "23jh23lkl".into(),
            vec![Hunk::new(vec![
                LINE::ADD {
                    number: 4,
                    line: "added line...".into(),
                },
                LINE::NOP {
                    number_left: 5,
                    number_right: 6,
                    line: "line...".into(),
                },
                LINE::REM {
                    number: 9,
                    line: "removed line...".into(),
                },
            ])],
        );

        assert_eq!(expected_output, print(&vec![file], None));
    }
}
