# spiv

<div align="center">

![Spiv](SpivLogoReadme.png)

**A cross-platform package manager wrapper**

</div>

---

## Overview

`spiv` is a lightweight, cross-platform CLI tool that provides a unified interface for common package manager operations. Instead of memorising commands for `apt`, `dnf`, `pacman`, or `winget`, `spiv` lets you use a single consistent syntax regardless of your OS or distro.

**Supported package managers:**
- **Linux:** `apt` (Debian/Ubuntu), `dnf` (Fedora/RHEL), `pacman` (Arch)
- **Windows:** `winget`

---

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/AYNSJC/spiv
cd spiv
cargo build --release
```

The compiled binary will be at `target/release/spiv`. You can move it to a directory on your `PATH`:

```bash
# Linux
sudo mv target/release/spiv /usr/local/bin/

# Windows (PowerShell, as admin)
Move-Item target\release\spiv.exe C:\Windows\System32\
```

---

## Usage

```
spiv [flag] [argument] [option]
```

### Flags

| Flag | Arguments | Description |
|------|-----------|-------------|
| `-f <term>` | search term | Search for a package |
| `-i <package>` | package name | Install a package |
| `-u /a` | — | Update all packages and package manager |
| `-u <package>` | package name | Update a specific package |
| `-r <package>` | package name | Remove a specific package |
| `-w` | — | Show the detected package manager |
| `-c` | — | Clear the terminal screen |
| `-h` | — | Show help |

### Options

| Option | Description |
|--------|-------------|
| `/c` | Cleaner, indexed output (currently Pacman only) |
| `/l<path>` | Install to a custom path — **Windows / MSI installers only** |
| `/a` | Refers to "all" (used with `-u`) |

---

## Examples

```bash
# Search for a package
spiv -f neovim

# Install a package
spiv -i git

# Install to a custom path (Windows)
spiv -i nodejs /lC:\Tools

# Update a specific package
spiv -u firefox

# Update everything
spiv -u /a

# Remove a package
spiv -r vlc

# Search with cleaner output (Pacman)
spiv -f python /c

# Check which package manager was detected
spiv -w
```

---

## How It Works

On startup, `spiv` detects the available package manager by checking for `apt`, `dnf`, or `pacman` in order. On Windows, it always delegates to `winget`. All operations are then transparently forwarded to the appropriate backend with the correct arguments.

---

## Dependencies

- [colored](https://crates.io/crates/colored) — terminal colour output

---

## License

GNU General Public License v3.0
