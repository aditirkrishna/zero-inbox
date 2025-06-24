# Zero Inbox

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/zero-inbox/zero-inbox/workflows/Rust/badge.svg)](https://github.com/zero-inbox/zero-inbox/actions)

> "Because your brain deserves a compiler, too."

---

## What is Zero Inbox?

Zero Inbox is a hand-crafted, slightly over-engineered, and definitely not AI-generated CLI tool for turning your chaotic, human-readable task lists into beautifully structured, machine-executable plans. It's a compiler. For your attention. In Rust. Because why not?

- **Minimalist DSL**: Write your day like code. Because you already think in lists.
- **Output options**: Shell scripts, Markdown, JSON, or even a calendar file. Impress your friends, confuse your enemies.
- **Cross-platform**: Windows, macOS, Linux. If it runs Rust, it runs Zero Inbox.

---

## Installation

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (because you like living on the edge)
- Git (for the source-code purists)

### Install the easy way
```sh
cargo install zibox
```

### Or build from source (for the control freaks)
```sh
git clone https://github.com/zero-inbox/zero-inbox.git
cd zero-inbox
cargo install --path .
```

- On Windows: Make sure `%USERPROFILE%\.cargo\bin` is in your PATH.
- On macOS/Linux: `$HOME/.cargo/bin` should already be in your PATH, unless you like surprises.

---

## Quick Start (Because Reading is for Later)

1. **Write a `.zbx` file:**
   ```
   @morning
     write(report) [2h]
     clear(inbox)

   drink(coffee) [15m]

   @evening
     reflect(day) [1h]
     read(book) [30m]
   ```
2. **Compile your plan:**
   ```sh
   zibox daily_plan.zbx
   ```
   By default, you get Markdown. Because everyone loves checklists.

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

## Platform Notes (No Excuses)

- **Windows:** Shell scripts need WSL, Git Bash, or a compatible terminal. Everything else works natively. Welcome to 2024.
- **macOS/Linux:** Everything just works. Unless it doesn't, in which case, blame your package manager.
- **Calendar:** `.ics` output works everywhere. If it doesn't, it's not our fault.

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

## Features (Because Bullet Points Are Fun)

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
- [AST Implementation](docs/AST_IMPLEMENTATION.md): For compiler nerds

---

## License

MIT. Because sharing is caring.

---

*Zero Inbox: Written by humans, for humans. No AI, no magic, just a lot of coffee and questionable life choices.*
