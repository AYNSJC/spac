use std::io;
use std::process::Command;

fn main() {
    let mut running = true;

    while running {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.contains("-f") {
            if let Some((_flag, query)) = input.split_once(' ') {
                if cfg!(target_os = "windows") {
                    let output = Command::new("cmd").args(["/C", &format!("winget search {}", query)]).output().expect("failed to execute process");

                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                println!("Usage: -f <search_term>");
            }
        }

        if input == "-q" {
            running = false;
        }
    }
}