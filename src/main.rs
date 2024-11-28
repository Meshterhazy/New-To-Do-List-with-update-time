use chrono::Local;
use std::fs::{read_to_string, write, OpenOptions};
use std::io::{self, Write};

fn main() {
    println!("Welcome to To-Do List!");

    println!("Write the name of file: ");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Name can't enter");
    filename
        .trim()
        .parse::<String>()
        .expect("File get ready for name check");

    loop {
        println!(
            "Write the number of operation: \n\
             (1) Add a note. \n\
             (2) Edit a note. \n\
             (3) Remove a note. \n\
             (4) Show all notes. \n\
             (5) Exit."
        );

        let mut operations = String::new();
        io::stdin()
            .read_line(&mut operations)
            .expect("Failed to read input");
        let operation: u8 = match operations.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number from 1 to 5.");
                continue;
            }
        };
        match operation {
            1 => add_notes(&mut filename),
            2 => edit_notes(&mut filename),
            3 => remove_notes(&mut filename),
            4 => show_notes(&mut filename),
            5 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid operation. Please choose a number from 1 to 5.");
            }
        }
    }
}

fn add_notes(filename: &str) {
    println!("Write your new notes: ");

    let mut new_notes = String::new();
    io::stdin()
        .read_line(&mut new_notes)
        .expect("Failed. Try again.");

    let now = Local::now();
    let _timelaps = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let note_with_time = format!("{} - {} \n", _timelaps, new_notes.trim());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Failed to open file");

    if let Err(e) = file.write_all(note_with_time.as_bytes()) {
        eprintln!("Error writing to file: {}", e)
    } else {
        println!("Note added.");
    }
}

fn edit_notes(filename: &str) {
    let content = match read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            eprint!("Notes not found.");
            return;
        }
    };

    let mut notes: Vec<&str> = content.lines().collect();

    if notes.is_empty() {
        println!("File have no notes");
        return;
    }

    println!("Enter the number of the note to edit: ");
    let mut number_for_edit = String::new();
    io::stdin()
        .read_line(&mut number_for_edit)
        .expect("Notes not found.");

    let index: usize = match number_for_edit.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= notes.len() => num - 1,
        _ => {
            println!("Invalid number");
            return;
        }
    };

    println!("Write correct note.");
    let mut new_notes = String::new();
    io::stdin()
        .read_line(&mut new_notes)
        .expect("Failed to read.");
    new_notes
        .trim()
        .parse::<String>()
        .expect("Write the correct notes");

    notes[index] = new_notes.trim();
    let updated_content = notes.join("\n");

    if let Err(e) = write(filename, updated_content) {
        eprint!("Error updating the file: {e}");
    } else {
        println!("Note edited.");
    }
}

fn remove_notes(filename: &str) {
    let content = match read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            eprint!("Number of notes not found.");
            return;
        }
    };

    let mut notes: Vec<&str> = content.lines().collect();

    if notes.is_empty() {
        println!("U have no notes for remove.");
        return;
    }

    println!("Your current notes:");
    for (index, note) in notes.iter().enumerate() {
        println!("{}. {note}", index + 1);
    }

    println!("Enter the number of the note to remove: ");
    let mut to_remove = String::new();
    io::stdin()
        .read_line(&mut to_remove)
        .expect("Invalid number of notes to remove");
    let index: usize = match to_remove.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= notes.len() => num - 1,
        _ => {
            println!("Invalid number.");
            return;
        }
    };

    notes.remove(index);
    let update_content = notes.join("\n");

    if let Err(e) = write(filename, update_content) {
        eprint!("Error updatig: {e}");
    } else {
        println!("Note removed.")
    }
}

fn show_notes(filename: &str) {
    let content = match read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            println!("Notes not found.");
            return;
        }
    };

    if content.trim().is_empty() {
        println!("Your To-Do-List are empty. Please write the new notes ");
    } else {
        println!("All your notes:\n {content}");
    }
}
