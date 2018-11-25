use std::collections::HashMap;
use file::{File, Hunk, MODIFIER};

pub fn parse_content(input: &Vec<String>) -> Vec<File> {
    let mut content: HashMap<i32, String> = HashMap::new();

    for i in 0..input.len() {
        content.insert(i as i32, input.get(i).unwrap().to_string());
    }

    vec![
        File::new(
            "filename.rs".to_string(), 
            "2jhg2323".to_string(), 
            vec![
            Hunk::new(
                MODIFIER::ADD,
                content.clone(),
                )
            ]
        ),
        File::new(
            "nextfile.rs".to_string(), 
            "9812i1u23".to_string(), 
            vec![
            Hunk::new(
                MODIFIER::DELETE,
                content.clone(),
                )
            ]
        )
    ]
}
