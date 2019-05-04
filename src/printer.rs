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

/// Main print method tfor printing the file content and the styling
///
/// # Arguments
///
/// * `files` - files that will be printed
///
pub fn print(files: &Vec<File>, _columnview: Option<&str>) {
    let terminal_size = term_size::dimensions();
    let term_width = terminal_size.unwrap_or((0, 0)).0;

    // for every file in the diff
    files.iter().for_each(|file| {
        // linenumber width
        let max_line_number = file.get_max_line_number_size();
        let ln_width = max_line_number.to_string().chars().count() + 3;

        // filename
        print_line(&term_width, &ln_width, LINE_ANCHOR_UP);
        print_filename(&file.modifier, &file.filename, &file.commit_id, &ln_width);
        print_line(&term_width, &ln_width, LINE_ANCHOR_MIDDLE);

        // hunks
        for i in 0..file.hunks.len() {
            for line in &file.hunks[i].content {
                print_line_content(&ln_width, &line);
            }
            if file.hunks.len() > 1 && file.hunks.len() - 1 != i {
                print_cut(&term_width);
            }
        }

        print_line(&term_width, &ln_width, LINE_ANCHOR_DOWN);
    });
}

/// Prints a horizontal line at the beginning, after the filename and at the
/// end of a file.
///
/// # Arguments
///
/// * `width` - the terminal width for line length
/// * `ln_width` - the width of the linenumbers column
/// * `indent_char` - the char to print at the indent for the vertical column
/// line
///
fn print_line(width: &usize, ln_width: &usize, indent_char: char) {
    for i in 1..*width {
        if i == *ln_width {
            print!(
                "{}",
                Colour::Fixed(FIXED_COLOUR).paint(indent_char.to_string())
            );
        }
        print!("{}", Colour::Fixed(FIXED_COLOUR).paint(LINE.to_string()));
    }
    println!();
}

/// Print a outline after every hunk in a file to show the cut in a file.
///
/// # Arguments
///
/// * `width` - the terminal width for line length
///
fn print_cut(width: &usize) {
    // down cut
    for _ in (1..*width).step_by(2) {
        print!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT1.to_string())
        );
        print!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT2.to_string())
        );
    }
    println!();

    // up cut
    for _ in (1..*width).step_by(2) {
        print!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT3.to_string())
        );
        print!(
            "{}",
            Colour::Fixed(FIXED_COLOUR).paint(LINE_CUT4.to_string())
        );
    }
    println!();
}

/// Print the filename in the header row of a file
///
/// # Arguments
///
/// * `modifier` - the git modifier (add, delete, ...)
/// * `filename` - the filename
/// * `commit_id` - commit id of the file
/// * `ln_width` - linenumber column width for indent
///
fn print_filename(modifier: &MODIFIER, filename: &str, commit_id: &str, ln_width: &usize) {
    let modifier_symbol = match modifier {
        MODIFIER::ADD => Colour::Green.bold().paint(MODIFIER_ADD.to_string()),
        MODIFIER::MODIFIED => Colour::Yellow.bold().paint(MODIFIER_MODIFIED.to_string()),
        MODIFIER::RENAMED => Colour::Purple.bold().paint(MODIFIER_MODIFIED.to_string()),
        MODIFIER::DELETE => Colour::Red.bold().paint(MODIFIER_DELETE.to_string()),
    };

    for _ in 1..*ln_width {
        print!(" ")
    }
    println!(
        "{} {} {} {}{}",
        Colour::Fixed(FIXED_COLOUR).paint("│"),
        modifier_symbol,
        Style::new().bold().paint(filename),
        Colour::Blue.bold().paint("@"),
        Colour::Blue.paint(commit_id),
    );
}

/// Line content with the different colours for the diff
///
/// # Arguments
///
/// * `ln_width` - linenumber column width for indent
/// * `line` - the line object with their modifiers and content
///
fn print_line_number(ln_width: &usize, line_number: &usize) {
    for mut i in 1..*ln_width {
        if i + line_number.to_string().chars().count() + 1 == *ln_width {
            print!(
                "{} ",
                Colour::Fixed(FIXED_COLOUR).paint(line_number.to_string())
            );
            break;
        } else {
            print!(" ");
        }
    }
    print!(
        "{}",
        Colour::Fixed(FIXED_COLOUR).paint(LINENUMBER_SEPERATOR.to_string())
    );
}

fn print_line_content(ln_width: &usize, line: &LINE) {
    match line {
        LINE::ADD((nr, line)) => {
            print_line_number(&ln_width, &nr);
            println!("{}", Colour::Green.paint(format!("+{}", line.to_string())))
        }
        LINE::REM((nr, line)) => {
            print_line_number(&ln_width, &nr);
            println!("{}", Colour::Red.paint(format!("-{}", line.to_string())))
        }
        LINE::NOP((_, nr_right, line)) => {
            print_line_number(&ln_width, &nr_right);
            println!("{}", Colour::White.paint(format!(" {}", line.to_string())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::file::Hunk;
    use super::*;
    #[test]
    #[ignore]
    fn print_file_test() {
        let file: File = File::new(
            MODIFIER::MODIFIED,
            "filename.rs".into(),
            "23jh23lkl".into(),
            vec![Hunk::new(vec![
                LINE::ADD((4, "added line...".into())),
                LINE::NOP((5, 6, "line...".into())),
                LINE::NOP((6, 7, "line...".into())),
                LINE::NOP((7, 8, "line...".into())),
                LINE::REM((9, "removed line...".into())),
                LINE::NOP((10, 11, "line...".into())),
            ])],
        );

        print(&vec![file], None)
    }
}
