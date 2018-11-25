use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum MODIFIER {
    ADD,
    MODIFIED,
    DELETE,
}

#[derive(Debug, Clone)]
pub struct Hunk {
    modifier: MODIFIER,
    content: HashMap<i32, String>,
}

impl Hunk {
    pub fn new(modifier: MODIFIER, content: HashMap<i32, String>) -> Hunk {
        Hunk {
            modifier: modifier,
            content: content,
        }
    }
}

impl fmt::Display for Hunk {    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MOD: {:?}\nContent: {:?}", self.modifier, self.content)
    }
}

#[derive(Debug, Clone)]
pub struct File {
    filename: String,
    commit_id: String,
    hunks: Vec<Hunk>
}

impl File {
    pub fn new(filename: String, commit_id: String, hunks: Vec<Hunk>) -> File {
        File {
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
            self.hunks.iter().for_each(|hunk| 
               hunk_str.push_str(&format!("{}", hunk))
            );
        }       
        write!(f, "Filename: {}\nCommit-ID: {}\n\n{}", self.filename, self.commit_id, hunk_str)
    }
}
