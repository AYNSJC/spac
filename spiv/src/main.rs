use std::env::args;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut running = true;

    while running {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        let mut parts = input.split_whitespace();

        let flag = parts.next();
        let arg = parts.next();

        match flag {
            Some("-f") => {
                if let Some(query) = arg {
                    search_package(query);
                }
                else {
                    println!("Usage: -f <search_term>");
                }
            }

            Some("-i") => {
                if let Some(package) = arg {
                    install_package(package);
                }
                else {
                    println!("Usage: -i <package_name>");
                }
            }

            Some("-u") => {
                update_packages(arg);
            }

            Some("-c") => {
                clear_screen();
            }

            Some("-h") => {
                print_help();
            }

            Some("-q") => {
                running = false;
            }

            Some(_) | None => {
                println!("Unknown command. Use -h for help.");
            }
        }
    }
}

fn search_package(query: &str) {
    if cfg!(target_os = "windows") {
        Command::new("winget").args(["search", query]).status().expect("failed to execute winget");
    }
    else if cfg!(target_os = "linux") {
        if command_exists("apt") {
            Command::new("apt").args(["search", query]).status().expect("failed to execute apt");
        }
        else if command_exists("dnf") {
            Command::new("dnf").args(["search", query]).status().expect("failed to execute dnf");
        }
        else if command_exists("pacman") {
            Command::new("pacman").args(["-Ss", query]).status().expect("failed to execute pacman");
        }
        else {
            println!("No supported package manager found.");
        }
    }
}

fn install_package(package: &str) {
    if cfg!(target_os = "windows") {
        Command::new("winget").args(["install", package]).status().expect("failed to execute winget");
    }
    else if cfg!(target_os = "linux") {
        if command_exists("apt") {
            Command::new("sudo").args(["apt", "install", "-y", package]).status().expect("failed to execute apt");
        }
        else if command_exists("dnf") {
            Command::new("sudo").args(["dnf", "install", "-y", package]).status().expect("failed to execute dnf");
        }
        else if command_exists("pacman") {
            Command::new("sudo").args(["pacman", "-S", "--noconfirm", package]).status().expect("failed to execute pacman");
        }
        else {
            println!("No supported package manager found.");
        }
    }
}

fn update_packages(arg: Option<&str>) {
    match arg {
        Some("/a") => {
            if cfg!(target_os = "windows") {
                Command::new("winget").args(["upgrade", "--all"]).status().expect("failed to execute winget");
            }
            else if cfg!(target_os = "linux") {
                if command_exists("apt") {
                    Command::new("sudo").args(["apt", "update"]).status().expect("failed to update apt");
                    Command::new("sudo").args(["apt", "upgrade", "-y"]).status().expect("failed to upgrade apt");
                }
                else if command_exists("dnf") {
                    Command::new("sudo").args(["dnf", "upgrade", "-y"]).status().expect("failed to execute dnf");
                }
                else if command_exists("pacman") {
                    Command::new("sudo").args(["pacman", "-Syu", "--noconfirm"]).status().expect("failed to execute pacman");
                }
                else {
                    println!("No supported package manager found.");
                }
            }
        }

        Some(pkg) => {
            if cfg!(target_os = "windows") {
                Command::new("winget").args(["upgrade", pkg]).status().expect("failed to execute winget");
            }
            else if cfg!(target_os = "linux") {
                if command_exists("apt") {
                    Command::new("sudo").args(["apt", "install", "--only-upgrade", pkg]).status().expect("failed to upgrade apt package");
                }
                else if command_exists("dnf") {
                    Command::new("sudo").args(["dnf", "upgrade", "-y", pkg]).status().expect("failed to upgrade dnf package");
                }
                else if command_exists("pacman") {
                    Command::new("sudo").args(["pacman", "-S", pkg, "--noconfirm"]).status().expect("failed to upgrade pacman package");
                }
                else {
                    println!("No supported package manager found.");
                }
            }
        }

        None => {
            println!("Usage: -u /a  (all)  OR  -u <package>");
        }
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    }
    else {
        Command::new("clear").status().unwrap();
    }
}

fn print_help() {
    println!("Welcome to spiv");
    println!("Commands:");
    println!("-f <search_term>          | Search for a package");
    println!("-i <package_name>         | Install a package");
    println!("-u <package_name>         | Updates all packages");
    println!("-c                        | Clear screen");
    println!("-h                        | Show help");
    println!("-q                        | Quit");
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which").arg(cmd).output().map(|o| o.status.success()).unwrap_or(false)
}