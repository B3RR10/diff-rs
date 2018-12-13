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

#[derive(Debug, Clone, Copy)]
pub enum MODIFIER {
    ADD,
    MODIFIED,
    RENAMED,
    DELETE,
    NOP,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub modifier: MODIFIER,
    pub line_number: usize,
    pub line: String,
}

#[derive(Debug, Clone)]
pub struct Hunk {
    pub content: Vec<Line>,
}

impl Hunk {
    pub fn new(content: Vec<Line>) -> Hunk {
        Hunk { content: content }
    }
}

impl fmt::Display for Hunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content: {:?}", self.content)
    }
}

#[derive(Debug, Clone)]
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
