# zero-inbox

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> "when your to-do list needs its own compiler"

---

## Introduction

Zero Inbox is what happens when a developer has one too many espresso shots and thinks: "My TODO list should compile." It's a CLI tool that turns your chaotic human thoughts into executable plans. Written in Rust because we like our memory safe and our compile times long.

- **Minimalist DSL**: Write your day like code. Because `buy(milk)` is objectively better than "remember to buy milk"
- **Output options**: Shell scripts, Markdown, JSON, or even a calendar file. Pick your poison.
- **Cross-platform**: Works everywhere Rust does (which is everywhere except your toaster... yet)

---

## Installation

### Prerequisites
- [Rust toolchain](https://rustup.rs/) - because living on the edge is more fun with memory safety
- Git - for those who like their source code like their coffee: fresh and bitter

### The Easy Way (Recommended for Normal Humans)
```sh
cargo install zibox
```

### From Source 
```sh
git clone https://github.com/zero-inbox/zero-inbox.git
cd zero-inbox
cargo install --path .
```

- On Windows: Make sure `%USERPROFILE%\.cargo\bin` is in your PATH.
- On macOS/Linux: `$HOME/.cargo/bin` should already be in your PATH, unless you like surprises.

---

## Quick Start (Before You Change Your Mind)

1. **Write a `.zbx` file** because we need more file extensions in our lives:
   ```
   @morning
     write(report) [2h]  # 2 hours? Optimistic.
     clear(inbox)        # As if

   drink(coffee) [15m]   # The most important task
   ```

2. **Compile your life choices**:
   ```sh
   zibox daily_plan.zbx
   ```
   Defaults to Markdown because checklists give us the illusion of control.

---

## Output Formats (Pick Your Poison)

- **Shell script:**
  ```sh
  zibox daily_plan.zbx --output-format shell --output-file schedule.sh
  # Linux/macOS: bash schedule.sh
  # Windows: Use WSL or Git Bash. Or just stare at the script.
  ```
- **Markdown:**
  ```sh
  zibox daily_plan.zbx --output-format markdown > plan.md
  ```
- **JSON:**
  ```sh
  zibox daily_plan.zbx --output-format json > plan.json
  ```
- **Calendar (.ics):**
  ```sh
  zibox daily_plan.zbx --calendar --output-file plan.ics
  # Import into Google Calendar, Outlook, or Apple Calendar. Or don't.
  ```

---

## Advanced Usage (For the Ambitious)

- **Set work hours:**
  ```sh
  zibox daily_plan.zbx --workday-start 09:00 --workday-end 18:00
  ```
- **Focus on tags:**
  ```sh
  zibox daily_plan.zbx --focus-tag deepwork
  ```
- **Visualize schedule (ASCII Gantt, because why not):**
  ```sh
  zibox daily_plan.zbx --visualize-schedule
  ```
- **Dry run (simulate, don't commit):**
  ```sh
  zibox daily_plan.zbx --dry-run
  ```
- **Show the IR (for the truly curious):**
  ```sh
  zibox daily_plan.zbx --show-ir
  ```
- **Custom config:**
  Drop a `.ziboxrc` file (TOML) in your home or project directory. Because you like to tinker.

---

## Platform Notes

- **Windows**: Requires WSL/Git Bash for shell scripts. Everything else works natively
- **macOS/Linux**: Should work out of the box. If not, check your Rust installation
- **Calendar**: Standard .ics format supported by most calendar apps

---

## Troubleshooting & FAQ (Read This Before Filing Issues)

- **Build errors?**
  - Close anything using `target/` (Windows loves file locks).
  - Run `cargo clean` and try again. Or just reboot, like it's 1998.
- **Shell script won't run?**
  - Use `bash schedule.sh` in a real terminal.
- **Notifications not working?**
  - Install `notify-send` (Linux) or use macOS's built-in notifications. Or just look at your screen.
- **More help?**
  - See [docs/USAGE.md](docs/USAGE.md). Or just experiment. What's the worst that could happen?

---

## Features (because bullet points are fun)

- Minimal, readable DSL for tasks
- Multiple output formats (shell, markdown, JSON, calendar)
- Task dependencies and priorities
- Tag-based filtering and optimization
- Configurable work hours and parallelism
- Visual schedule output (for your inner artist)
- Cross-platform, no external dependencies required for core features

---

## Documentation

- [Project Steps](steps.md): The full, slightly obsessive development plan
- [Usage Guide](docs/USAGE.md): Step-by-step instructions and advanced examples
- [AST Implementation](docs/ast-guide.md): For those who like their parsers with extra caffeine

---

## License

MIT licensed. Use responsibly - we're not liable for any productivity gains (or losses).

---

<p align="center"><i>built with ❤️ by aditi ramakrishnan</i></p>