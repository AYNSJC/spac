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
                else if cfg!(target_os = "linux") {
                    if command_exists("apt") {
                        Command::new("apt").args(["search", query]).status().expect("failed to execute apt");
                    } else if command_exists("dnf") {
                        Command::new("dnf").args(["search", query]).status().expect("failed to execute dnf");
                    } else if command_exists("pacman") {
                        Command::new("pacman").args(["-Ss", query]).status().expect("failed to execute pacman");
                    } else {
                        println!("No supported package manager found.");
                    }
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
        else if input.contains("-h") {
            println!("Welcome to spiv");
            println!("Here are a few common commands");
            println!("spiv -f <search_term>  searches for the given package");
            println!("spiv -h  Gives common commands");
            println!("spiv -c  Clears screen");
            println!("spiv -q  quits the program");
        }
        else if input == "-q" {
            running = false;
        }
    }
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}