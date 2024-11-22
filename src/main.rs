use std::fs::File;
use std::io::{self, Write};

fn main() {
    println!("Wellome to To-Do List!");
    println!("Write the notes for today or yesterday!");

    let filename = "notes.txt";
    let mut _file = match File::create(filename) {
        Ok(y) => y,
        Err(n) => {
            eprintln!("Error creating file: {}", n);
            return;
        }
    };

    let mut todo_item = String::new();
    io::stdin()
        .read_line(&mut todo_item)
        .expect("New note are failed.");
    todo_item
        .trim()
        .parse::<String>()
        .expect("File get ready to enter");

    if let Err(n) = _file.write_all(todo_item.as_bytes()) {
        eprint!("To-Do-File new item: {}", todo_item);
    }
}
