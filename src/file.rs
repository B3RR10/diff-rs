//! Main modul to persist the file objects in a diff output. Contains some
//! import structs to define the different parts of a diff
//!
//! Main struct is `File` with the git modifier (add, delete, rename, ...),
//! the commit id, filename and there hunks.
//!
//! A `Hunk` contains the lines (`Line`) with there diffs and also their
//! modifieres.
//!
//! The lines (`Line`) consist of their numbers, modifiers and the linecontent.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MODIFIER {
    ADD,
    MODIFIED,
    RENAMED,
    DELETE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LINE {
    ADD {
        number: usize,
        line: String,
    },
    REM {
        number: usize,
        line: String,
    },
    NOP {
        number_left: usize,
        number_right: usize,
        line: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hunk {
    pub content: Vec<LINE>,
}

impl Hunk {
    pub fn new(content: Vec<LINE>) -> Hunk {
        Hunk { content: content }
    }
}

impl fmt::Display for Hunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content: {:?}", self.content)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub modifier: MODIFIER,
    pub filename: String,
    pub commit_id: String,
    pub hunks: Vec<Hunk>,
}

impl File {
    pub fn new(modifier: MODIFIER, filename: String, commit_id: String, hunks: Vec<Hunk>) -> File {
        File {
            modifier: modifier,
            filename: filename,
            commit_id: commit_id,
            hunks: hunks,
        }
    }

    pub fn get_max_line_number_size(&self) -> usize {
        self.hunks
            .iter()
            .map(|hunk| {
                hunk.content
                    .iter()
                    .map(|line| match line {
                        LINE::ADD {
                            number: nr,
                            line: _,
                        } => nr,
                        LINE::REM {
                            number: nr,
                            line: _,
                        } => nr,
                        LINE::NOP {
                            number_left: nr1,
                            number_right: nr2,
                            line: _,
                        } => {
                            if nr1 > nr2 {
                                nr1
                            } else {
                                nr2
                            }
                        }
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
            .clone()
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hunk_str = String::new();
        if self.hunks.len() > 0 {
            hunk_str.push_str("Hunks: \n");
            self.hunks
                .iter()
                .for_each(|hunk| hunk_str.push_str(&format!("{}", hunk)));
        }
        write!(
            f,
            "Filename: {}\nCommit-ID: {}\n\n{}",
            self.filename, self.commit_id, hunk_str
        )
    }
}
