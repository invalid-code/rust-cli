use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fs;
use std::path::Path;
use std::str::from_utf8;

#[derive(Clone, ValueEnum, Debug, Default)]
enum Command {
    #[default]
    Add,
    List,
    Show,
    Remove,
}

#[derive(Parser, Debug, Default)]
struct Cli {
    command: Command,
    #[clap(default_value_t = String::new())]
    value: String,
    #[clap(short, long)]
    key: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    key: i32,
    body: String,
}

fn add(counter: &mut i32, args: Cli, path: &str, mut notes: Vec<Note>) {
    counter += 1;
    let new_note = Note {
        key: counter,
        body: args.value,
    };
    println!("{:?}", &new_note);
    notes.push(new_note);
    fs::write(path, &to_string(&notes).unwrap()).unwrap();
}

fn list(notes: Vec<Note>) -> () {
    println!("Notes:");
    for note in notes {
        println!("  {}", note.body);
    }
}

fn show(notes: Vec<Note>, key: i32, args: Cli) {
    match args.key {
        Some(key) => {
            for note in notes {
                if note.key == key {
                    println!("{}", note.body);
                }
            }
        }
        None => println!("Pls input a key"),
    }
}

fn main() {
    const PATH: &str = "notes.json";
    if !Path::new(PATH).exists() {
        fs::write(PATH, b"[]").unwrap();
    }
    let args = Cli::parse();
    match fs::read(PATH) {
        Ok(y) => {
            let notes_str = from_utf8(&y).expect("Error parsing file");
            let mut notes: Vec<Note> = serde_json::from_str(notes_str).unwrap();
            let mut counter = notes[notes.len() - 1].key;
            match &args.command {
                Command::Add => add(&mut counter, args, PATH, notes),
                Command::List => list(notes),
                Command::Show => show(notes, counter, args),
                Command::Remove => match args.key {
                    Some(key) => {
                        let mut to_remove: Option<usize> = None;
                        for (note_index, note) in notes.iter().enumerate() {
                            if note.key == key {
                                to_remove = Some(note_index);
                            }
                        }
                        if let Some(remove) = to_remove {
                            notes.remove(remove);
                        }
                    }
                    None => println!("Pls input a key"),
                },
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
