use std::env;
use std::process::Command;
use colored::{control, Colorize};

fn main() {
    let mut args = env::args().skip(1);

    let flag = args.next();
    let arg = args.next();
    let extra = args.next();

    #[cfg(target_os = "windows")] {
        control::set_virtual_terminal(true).unwrap();
    }

    match flag.as_deref() {
        Some("-f") => {
            if let Some(query) = arg.as_deref() {
                search_package(query);
            }
            else {
                println!("{}", "Usage: -f <search_term>".yellow());
            }
        }
        Some("-i") => {
            if let Some(package) = arg.as_deref() {
                install_package(package, extra.as_deref());
            }
            else {
                println!("{}", "Usage: -i <package_name> [/l<path>]".yellow());
            }
        }
        Some("-u") => {
            update_packages(arg.as_deref(), extra.as_deref());
        }
        Some("-r") => {
            if let Some(package) = arg.as_deref() {
                remove_package(package);
            }
            else {
                println!("{}", "Usage: -r <package_name>".yellow());
            }
        }
        Some("-c") => {
            clear_screen();
        }
        Some("-h") => {
            print_help();
        }
        Some(_) | None => {
            println!("{}", "Unknown command. Use -h for help.".yellow());
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
    println!("Searching {}", query.yellow().bold());

    #[cfg(target_os = "windows")] {
        Command::new("winget").args(["search", query]).status().expect("failed to execute winget");
    }

    #[cfg(target_os = "linux")] {
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
            println!("{}", "No supported package manager found.".red().bold());
        }
    }
}

fn install_package(package: &str, extra: Option<&str>) {
    println!("Installing {}", package.yellow().bold());

    let location = get_location(extra);

    #[cfg(target_os = "linux")] {
        if location.is_some() {
            println!("{} {}", "Warning: ".red().bold(), "Location flag is not supported on Linux package managers.".red().italic());
        }
    }

    #[cfg(target_os = "windows")] {
        if let Some(loc) = &location {
            Command::new("winget").args(["install", package, "--location", loc]).status().expect("failed to execute winget");
        }
        else {
            Command::new("winget").args(["install", package]).status().expect("failed to execute winget");
        }
    }

    #[cfg(target_os = "linux")] {
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
            println!("{}", "No supported package manager found.".red().bold());
        }
    }
}

fn remove_package(package: &str) {
    println!("Removing {}", package.yellow().bold());

    #[cfg(target_os = "windows")] {
        Command::new("winget").args(["uninstall", package]).status().expect("failed to execute winget");
    }

    #[cfg(target_os = "linux")] {
        if command_exists("apt") {
            Command::new("sudo").args(["apt", "uninstall", "-y", package]).status().expect("failed to execute apt");
        }
        else if command_exists("dnf") {
            Command::new("sudo").args(["dnf", "uninstall", "-y", package]).status().expect("failed to execute dnf");
        }
        else if command_exists("pacman") {
            Command::new("sudo").args(["pacman", "-R", "--noconfirm", package]).status().expect("failed to execute pacman");
        }
        else {
            println!("{}", "No supported package manager found.".red().bold());
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
            #[cfg(target_os = "windows")] {
                println!("{} {} {} {}", "Updating", "package manager".yellow().bold(), "and", "system files".yellow().bold());

                if let Some(loc) = &location {
                    Command::new("winget").args(["upgrade", "--all", "--location", loc]).status().expect("failed to execute winget");
                }
                else {
                    Command::new("winget").args(["upgrade", "--all"]).status().expect("failed to execute winget");
                }
            }

            #[cfg(target_os = "linux")] {
                update_all_linux();
            }
        }
        Some(pkg) => {
            println!("Updating {}", pkg.yellow().bold());

            #[cfg(target_os = "windows")] {
                if let Some(loc) = &location {
                    Command::new("winget").args(["upgrade", pkg, "--location", loc]).status().expect("failed to execute winget");
                }
                else {
                    Command::new("winget").args(["upgrade", pkg]).status().expect("failed to execute winget");
                }
            }
            #[cfg(target_os = "linux")] {
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
                    println!("{}", "No supported package manager found.".red().bold());
                }
            }
        }
        None => {
            println!("{}", "Usage: -u /a [/l<path>]  (all)  OR  -u <package> [/l<path>]".yellow());
        }
    }
}

fn update_all(location: Option<&str>) {
    #[cfg(target_os = "windows")] {
        println!("{} {} {} {}", "Updating", "package manager".yellow().bold(), "and", "system files".yellow().bold());

        if let Some(loc) = location {
            Command::new("winget").args(["upgrade", "--all", "--location", loc]).status().expect("failed to execute winget");
        }
        else {
            Command::new("winget").args(["upgrade", "--all"]).status().expect("failed to execute winget");
        }
    }

    #[cfg(target_os = "linux")] {
        update_all_linux();
    }
}

fn clear_screen() {
    #[cfg(target_os = "windows")] {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    }

    #[cfg(target_os = "linux")] {
        Command::new("clear").status().unwrap();
    }
}

fn print_help() {
    println!("{}", "Welcome to spiv".blue().bold());
    println!("Commands:                      |");
    println!("-f <search_term>               |{}", " Search for a package".yellow());
    println!("-i <package_name> [/l<path>]   |{}", " Install a package".yellow());
    println!("-u /a [/l<path>]               |{}", " Update all packages".yellow());
    println!("-u <package_name> [/l<path>]   |{}", " Update a specific package".yellow());
    println!("-r <package_name>              |{}", " Removes a specific package".yellow());
    println!("-c                             |{}", " Clear screen".yellow());
    println!("-h                             |{}", " Show help".yellow());
    println!("-q                             |{}", " Quit".yellow());
    println!("/l                             |{}", " Choose location to install/update to...".yellow());
    println!("/l only works for MSI installer|{}", " Warning".red().bold());
    println!("/a                             |{}", " Refers to all".yellow());
}

fn command_exists(cmd: &str) -> bool {
    println!("Checking package manager {}", cmd.yellow().bold());

    Command::new(cmd).arg("--version").output().is_ok()
}

fn update_all_linux() {
    println!("{} {} {} {}", "Updating", "package manager".yellow().bold(), "and", "system files".yellow().bold());

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
        println!("{}", "No supported package manager found.".red().bold());
    }
}