# pmcli — Project Manager CLI

[![Latest Release](https://img.shields.io/github/v/release/djunekz/pmcli?style=for-the-badge&logo=github&logoColor=white)](https://github.com/djunekz/pmcli/releases)
[![License](https://img.shields.io/github/license/djunekz/pmcli?style=for-the-badge&logo=open-source-initiative&logoColor=white)](https://github.com/djunekz/pmcli/blob/main/LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/djunekz/pmcli/rust.yml?branch=master&style=for-the-badge&logo=rust&logoColor=white)](https://github.com/djunekz/pmcli/actions)
[![Downloads](https://img.shields.io/github/downloads/djunekz/pmcli/total?style=for-the-badge&logo=protocols.io&logoColor=white)](https://github.com/djunekz/pmcli/releases)
[![Termux](https://img.shields.io/badge/Platform-Termux-brightgreen?style=for-the-badge&logo=android&logoColor=white)](https://termux.com/)
[![Linux](https://img.shields.io/badge/Platform-Linux-blue?style=for-the-badge&logo=linux&logoColor=white)](https://www.kernel.org/)
[![TUI Ready](https://img.shields.io/badge/TUI-Yes-purple?style=for-the-badge&logo=heroUI&logoColor=white)](https://github.com/djunekz/pmcli)
[![Git Sync Ready](https://img.shields.io/badge/GitSync-Yes-orange?style=for-the-badge&logo=git&logoColor=white)](https://github.com/djunekz/pmcli)
[![Rust Version](https://img.shields.io/badge/Rust-Stable-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

pmcli is a **production-grade project and task management CLI** written in Rust.
It is designed for developers and power users who work primarily in the terminal,
especially on **Termux and Linux** environments.

pmcli focuses on **simplicity, transparency, and Git-friendly workflows**.
All data is stored locally in human-readable files — no database, no lock-in.

---

## Why pmcli?

- CLI-first workflow
- Fast single binary (Rust)
- No database, only files
- Easy to version with Git
- Works perfectly on Termux
- Predictable and scriptable

---

## Features

- Project management
- Task management (add, list, complete)
- Task priorities (low / medium / high)
- Deadlines with date parsing
- Markdown notes per project
- Global configuration (TOML)
- Interactive TUI (Terminal UI)
- Git integration (init / commit / push / pull)
- Export project data

---

## Installation (Termux)

```bash
pkg install rust git
git clone https://github.com/djunekz/pmcli
cd pmcli
cargo build --release
cp target/release/pmcli $PREFIX/bin/
```

---

## Basic Usage

```bash
pmcli create myproject

pmcli add-task myproject "Design architecture" \
  --priority high \
  --deadline 2026-02-01

pmcli tasks myproject

pmcli done-task myproject 1

pmcli tui myproject
```

---

## Typical Workflow

1. Create a project
2. Add tasks with priority and deadline
3. Track progress via CLI or TUI
4. Store everything locally
5. Sync project directory using Git
6. Export data when needed

---

## TUI (Terminal UI)

```bash
pmcli tui myproject
```

TUI features:
- Live task filtering
- Split panel (task list + task details)
- Keyboard-driven navigation
- No mouse required

---

## Configuration

Configuration file location:

```text
~/.config/pmcli/config.toml
```

Example configuration:

```toml
default_priority = "medium"
date_format = "%Y-%m-%d"
```

---

## Git Synchronization

Each project can be a local Git repository.

```bash
pmcli git-init myproject
pmcli git-commit myproject "Update tasks"
pmcli git-push myproject
pmcli git-pull myproject
```

Authentication uses standard Git credential helpers or tokens.

---

## Data Layout

All data is stored locally:

```text
~/.pmcli/<project>/
```

Files inside each project:

- project.json   — project metadata
- tasks.json     — task list
- notes.md       — project notes

All files are human-readable and Git-friendly.

---

## Roadmap

- Task dependencies
- Recurring tasks
- Advanced TUI views
- Plugin system
- Official Termux package
- Documentation website

---

## FAQ

**Does pmcli require internet?**  
No. Internet is only required for Git sync.

**Is my data safe?**  
Yes. All data is stored locally in plain files.

**Is pmcli opinionated?**  
Yes. It favors simplicity and transparency over complexity.

---

## Contributing

Contributions are welcome.
Please open an issue before submitting large changes.

---

## License

[LICENSE](https://github.com/djunekz/pmcli/LICENSE)MIT License © 2026
