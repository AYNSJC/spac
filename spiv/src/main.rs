use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut running = true;

    while running {
        let mut input = String::new();

        print!(">");

        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.contains("-f") {
            if let Some((_flag, query)) = input.split_once(' ') {
                if cfg!(target_os = "windows") {
                    Command::new("winget").args(["search", query]).status().expect("failed to execute process");
                }
            } else {
                println!("Usage: -f <search_term>");
            }
        }
        else if input.contains("-c") {
            if cfg!(target_os = "windows") {
                Command::new("cmd").args(["/C", "cls"]).status().unwrap();
            }
        }
        else if input == "-q" {
            running = false;
        }
    }
}