use std::env;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use colored::{control, Colorize};

#[derive(Clone, Copy, PartialEq)]
enum PkgMan {
    Unknown,
    Apt,
    Dnf,
    Pacman
}

fn main() {
    let mut args = env::args().skip(1);

    let flag = args.next();
    let arg = args.next();
    let extra = args.next();

    #[cfg(target_os = "windows")] {
        control::set_virtual_terminal(true).unwrap();
    }

    let pkgman = {
        #[cfg(target_os = "linux")]{
            find_package_manager()
        }

        #[cfg(not(target_os = "linux"))] {
            PkgMan::Unknown
        }
    };

    match flag.as_deref() {
        Some("-f") => {
            if let Some(query) = arg.as_deref() {
                search_package(query, extra.as_deref(), pkgman);
            }
            else {
                println!("{}", "Usage: -f <search_term>".yellow());
            }
        }

        Some("-i") => {
            if let Some(package) = arg.as_deref() {
                install_package(package, extra.as_deref(), pkgman);
            }
            else {
                println!("{}", "Usage: -i <package_name> [/l<path>]".yellow());
            }
        }

        Some("-u") => {
            update_packages(arg.as_deref(), extra.as_deref(), pkgman);
        }

        Some("-r") => {
            if let Some(package) = arg.as_deref() {
                remove_package(package, pkgman);
            }
            else {
                println!("{}", "Usage: -r <package_name>".yellow());
            }
        }

        Some("-c") => {
            clear_screen();
        }

        Some("-w") => {
            #[cfg(target_os = "linux")]{
                get_package_manager(pkgman);
            }

            #[cfg(target_os = "windows")]{
                println!("{}", "winget".yellow());
            }
        }

        Some("-h") => {
            print_help();
        }

        Some(_) | None => {
            println!("{}", "Unknown command. Use -h for help.".yellow());
        }
    }
}

fn find_package_manager() -> PkgMan {
    if command_exists("apt") {
        PkgMan::Apt
    }
    else if command_exists("dnf") {
        PkgMan::Dnf
    }
    else if command_exists("pacman") {
        PkgMan::Pacman
    }
    else {
        println!("{}", "No supported package manager found.".red().bold());
        PkgMan::Unknown
    }
}

fn command_exists(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
}

fn search_package(query: &str, extra: Option<&str>, pkgman: PkgMan) {
    println!("Searching {}", query.yellow().bold());

    #[cfg(target_os = "windows")]
    {
        Command::new("winget").args(["search", query]).status().expect("failed to execute winget");
    }

    #[cfg(target_os = "linux")]
    {
        if pkgman == PkgMan::Apt {
            Command::new("apt").args(["search", query]).status().expect("failed to execute apt");
        }
        else if pkgman == PkgMan::Dnf {
            Command::new("dnf").args(["search", query]).status().expect("failed to execute dnf");
        }
        else if pkgman == PkgMan::Pacman {
            let mut raw = Command::new("pacman").args(["-Ss", query]).stdout(Stdio::piped()).spawn().expect("failed to execute pacman");

            let stdout = raw.stdout.take().expect("failed to take stdout");
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let output = line.expect("failed to read line");
                let mut index: i16 = 0;

                if let Some("/c") = extra {
                    if output.starts_with("core/") {
                        let cleaned = output.replace("core/", "");
                        println!("{index}. {}", cleaned.bold());
                        index += 1;
                    } else if output.starts_with("extra/") {
                        let cleaned = output.replace("extra/", "");
                        println!("{index}. {}", cleaned.bold());
                        index += 1;
                    } else if output.starts_with("community/") {
                        let cleaned = output.replace("community/", "");
                        println!("{index}. {}", cleaned.bold());
                        index += 1;
                    } else if output.starts_with("    ") {
                        let cleaned = output.replace("    ", ">  ");
                        println!("{}", cleaned.italic());
                        println!();
                    } else {
                        println!("{}", output);
                    }
                }
                else {
                    println!("{}", output);
                }
            }
        }
        else {
            println!("{}", "No supported package manager found.".red().bold());
        }
    }
}

fn install_package(package: &str, extra: Option<&str>, pkgman: PkgMan) {
    println!("Installing {}", package.yellow().bold());

    #[cfg(target_os = "windows")]
    {
        if let Some(loc) = get_location(extra) {
            Command::new("winget").args(["install", package, "--location", &loc]).status().expect("failed to execute winget");
        }
        else {
            Command::new("winget").args(["install", package]).status().expect("failed to execute winget");
        }
    }

    #[cfg(target_os = "linux")]
    {
        if pkgman == PkgMan::Apt {
            Command::new("sudo").args(["apt", "install", "-y", package]).status().expect("failed to execute apt");
        }
        else if pkgman == PkgMan::Dnf {
            Command::new("sudo").args(["dnf", "install", "-y", package]).status().expect("failed to execute dnf");
        }
        else if pkgman == PkgMan::Pacman {
            Command::new("sudo").args(["pacman", "-S", "--noconfirm", package]).status().expect("failed to execute pacman");
        }
        else {
            println!("{}", "No supported package manager found.".red().bold());
        }
    }
}

fn remove_package(package: &str, pkgman: PkgMan) {
    println!("Removing {}", package.yellow().bold());

    #[cfg(target_os = "windows")]
    {
        Command::new("winget").args(["uninstall", package]).status().expect("failed to execute winget");
    }

    #[cfg(target_os = "linux")]
    {
        if pkgman == PkgMan::Apt {
            Command::new("sudo").args(["apt", "remove", "-y", package]).status().expect("failed to execute apt");
        }
        else if pkgman == PkgMan::Dnf {
            Command::new("sudo").args(["dnf", "remove", "-y", package]).status().expect("failed to execute dnf");
        }
        else if pkgman == PkgMan::Pacman {
            Command::new("sudo").args(["pacman", "-R", "--noconfirm", package]).status().expect("failed to execute pacman");
        }
        else {
            println!("{}", "No supported package manager found.".red().bold());
        }
    }
}

fn update_packages(arg: Option<&str>, _extra: Option<&str>, pkgman: PkgMan) {
    match arg {
        Some("/a") => {
            update_all_linux(pkgman);
        }

        Some(pkg) => {
            println!("Updating {}", pkg.yellow().bold());

            #[cfg(target_os = "windows")]
            {
                Command::new("winget").args(["upgrade", pkg]).status().expect("failed to execute winget");
            }

            #[cfg(target_os = "linux")]
            {
                if pkgman == PkgMan::Apt {
                    Command::new("sudo").args(["apt", "install", "--only-upgrade", pkg]).status().expect("failed to upgrade apt");
                }
                else if pkgman == PkgMan::Dnf {
                    Command::new("sudo").args(["dnf", "upgrade", "-y", pkg]).status().expect("failed to upgrade dnf");
                }
                else if pkgman == PkgMan::Pacman {
                    Command::new("sudo").args(["pacman", "-S", pkg, "--noconfirm"]).status().expect("failed to upgrade pacman");
                }
                else {
                    println!("{}", "No supported package manager found.".red().bold());
                }
            }
        }

        None => {
            println!("{}", "Usage: -u /a  OR  -u <package>".yellow());
        }
    }
}

fn update_all_linux(pkgman: PkgMan) {
    println!("{} {} {} {}", "Updating", "package manager".yellow().bold(), "and", "packages".yellow().bold());

    if pkgman == PkgMan::Apt {
        Command::new("sudo").args(["apt", "update"]).status().expect("failed to update apt");
        Command::new("sudo").args(["apt", "upgrade", "-y"]).status().expect("failed to upgrade apt");
    }
    else if pkgman == PkgMan::Dnf {
        Command::new("sudo").args(["dnf", "upgrade", "-y"]).status().expect("failed to execute dnf");
    }
    else if pkgman == PkgMan::Pacman {
        Command::new("sudo").args(["pacman", "-Syu", "--noconfirm"]).status().expect("failed to execute pacman");
    }
    else {
        println!("{}", "No supported package manager found.".red().bold());
    }
}

fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("clear").status().unwrap();
    }
}

fn print_help() {
    println!("{}", "Welcome to spiv".blue().bold());
    println!("Commands:                        |");
    println!("-f <search_term>                 |{}", " Search for a package".yellow());
    println!("-i <package_name> [/l<path>]     |{}", " Install a package".yellow());
    println!("-u /a [/l<path>]                 |{}", " Update all packages".yellow());
    println!("-u <package_name> [/l<path>]     |{}", " Update a specific package".yellow());
    println!("-r <package_name>                |{}", " Removes a specific package".yellow());
    println!("-w                               |{}", " Tells the recognised package manager".yellow());
    println!("-c                               |{}", " Clear screen".yellow());
    println!("-h                               |{}", " Show help".yellow());
    println!("/c                               |{}", " Cleaner UI".yellow());
    println!("/l                               |{}", " Choose location (Windows ONLY)...".yellow());
    println!("/l only works for MSI installer  |{}", " Warning".red().bold());
    println!("/a                               |{}", " Refers to all".yellow()); }

fn get_location(token: Option<&str>) -> Option<String> {
    if let Some(t) = token {
        if t.starts_with("/l") {
            return Some(t[2..].to_string());
        }
    }
    None
}

fn get_package_manager(pkgman: PkgMan) {
    if pkgman == PkgMan::Apt {
        println!("{}", "apt recognised".yellow());
    }
    else if pkgman == PkgMan::Dnf {
        println!("{}", "dnf recognised".yellow());
    }
    else if pkgman == PkgMan::Pacman {
        println!("{}", "pacman recognised".yellow());
    }
    else {
        println!("{}", "Unknown package manager".red().bold());
    }
}