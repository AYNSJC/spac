use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);

    let flag = args.next();
    let arg = args.next();
    let extra = args.next();

    match flag.as_deref() {
        Some("-f") => {
            if let Some(query) = arg.as_deref() {
                search_package(query);
            }
            else {
                println!("Usage: -f <search_term>");
            }
        }
        Some("-i") => {
            if let Some(package) = arg.as_deref() {
                install_package(package, extra.as_deref());
            }
            else {
                println!("Usage: -i <package_name> [/l<path>]");
            }
        }
        Some("-u") => {
            update_packages(arg.as_deref(), extra.as_deref());
        }
        Some("-c") => {
            clear_screen();
        }
        Some("-h") => {
            print_help();
        }
        Some(_) | None => {
            println!("Unknown command. Use -h for help.");
        }
    }
}

fn get_location(token: Option<&str>) -> Option<String> {
    if let Some(t) = token {
        if t == "/l" {
            let current = env::current_dir().expect("Failed to get current directory");
            return Some(current.to_string_lossy().into_owned());
        }
        else if t.starts_with("/l") {
            return Some(t[2..].to_string());
        }
    }
    None
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

fn install_package(package: &str, extra: Option<&str>) {
    let location = get_location(extra);

    if cfg!(target_os = "windows") {
        if let Some(loc) = &location {
            Command::new("winget").args(["install", package, "--location", loc]).status().expect("failed to execute winget");
        }
        else {
            Command::new("winget").args(["install", package]).status().expect("failed to execute winget");
        }
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

fn update_packages(arg: Option<&str>, extra: Option<&str>) {
    if let Some(a) = arg {
        if a == "/l" || a.starts_with("/l") {
            let location = get_location(arg);
            update_all(location.as_deref());
            return;
        }
    }

    let location = get_location(extra);

    match arg {
        Some("/a") => {
            if cfg!(target_os = "windows") {
                if let Some(loc) = &location {
                    Command::new("winget").args(["upgrade", "--all", "--location", loc]).status().expect("failed to execute winget");
                }
                else {
                    Command::new("winget").args(["upgrade", "--all"]).status().expect("failed to execute winget");
                }
            }
            else if cfg!(target_os = "linux") {
                update_all_linux();
            }
        }
        Some(pkg) => {
            if cfg!(target_os = "windows") {
                if let Some(loc) = &location {
                    Command::new("winget").args(["upgrade", pkg, "--location", loc]).status().expect("failed to execute winget");
                }
                else {
                    Command::new("winget").args(["upgrade", pkg]).status().expect("failed to execute winget");
                }
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
            println!("Usage: -u /a [/l<path>]  (all)  OR  -u <package> [/l<path>]");
        }
    }
}

fn update_all(location: Option<&str>) {
    if cfg!(target_os = "windows") {
        if let Some(loc) = location {
            Command::new("winget").args(["upgrade", "--all", "--location", loc]).status().expect("failed to execute winget");
        }
        else {
            Command::new("winget").args(["upgrade", "--all"]).status().expect("failed to execute winget");
        }
    }
    else if cfg!(target_os = "linux") {
        update_all_linux();
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
    println!("Commands:                      |");
    println!("-f <search_term>               | Search for a package");
    println!("-i <package_name> [/l<path>]   | Install a package");
    println!("-u /a [/l<path>]               | Update all packages");
    println!("-u <package_name> [/l<path>]   | Update a specific package");
    println!("-c                             | Clear screen");
    println!("-h                             | Show help");
    println!("-q                             | Quit");
    println!("/l                             | Choose location to install/update to...");
    println!("/l only works for MSI installer| Warning");
    println!("/a                             | Refers to all");
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which").arg(cmd).output().map(|o| o.status.success()).unwrap_or(false)
}

fn update_all_linux() {
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