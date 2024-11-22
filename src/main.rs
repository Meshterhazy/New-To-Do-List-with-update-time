use chrono::Local;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    println!("Wellome to To-Do List!");

    loop {
        println!(
    "Write the number of operation: \n (1) Add notes. \n (2) Remove notes.  \n (3) Edit notes. \n (4) Show all notes. \n (5) Exit."
    );

        let filename = "notes.txt";
        let mut _file = match File::create(filename) {
            Ok(y) => y,
            Err(n) => {
                eprintln!("Error creating file: {}", n);
                return;
            }
        };

        let mut operations = String::new();
        io::stdin()
            .read_line(&mut operations)
            .expect("Operation sellection error");
        let operation: u8 = match operations.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Try again.");
                continue;
            }
        };

        println!("Write the notes.");

        let now = Local::now();
        let _timelaps = now.format("%Y-%m-%d %H:%M:%S").to_string();

        match operation {
            1 => add_notes(filename),
            2 => edit_notes(filename),
            3 => remove_notes(filename),
            4 => show_notes(filename),
            5 => {
                println!("U're exited from notex.txt .");
                break;
            }
            _ => {
                println!("Invalid operation. Try again.");
            }
        }
    }
}

fn add_notes(filename: &str) {
    let mut new_notes = String::new();
    io::stdin()
        .read_line(&mut new_notes)
        .expect("Failed. Try again.");
    new_notes
        .trim()
        .parse::<String>()
        .expect("Notes get ready to write.");
}

fn edit_notes(filename: &str) {}

fn remove_notes(filename: &str) {}

fn show_notes(filename: &str) {}
