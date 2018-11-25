use file::{File};

pub fn print(files: &Vec<File>) {
    files.iter().for_each(|file| {
       println!("==================================================");
       println!("{}\n", file);
       println!("==================================================");
    });
}
