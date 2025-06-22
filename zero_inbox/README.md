# Zero Inbox Compiler

A DSL compiler for attention/task modeling that turns structured text files into optimized execution plans.

## Overview

Zero Inbox is a minimalist compiler that transforms human-readable task definitions into structured, optimized, and executable plans. It's built in Rust with a focus on precision, minimalism, and developer experience.

## Features

- **Simple DSL** for defining tasks and blocks
- **Lexer and Pser** for processing the DSL
- **Intermediate Representation** for optimization
- **Multiple Output Formats** (Shell, Markdown, JSON)
- **Task Scheduling** with priorities and dependencies
- **Time Estimation** for better planning

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/zero_inbox.git
cd zero_inbox

# Build the project
cargo build --release

# Install globally
cargo install --path .
```

## Usage

1. Create a `.zbx` file with your tasks:

```
@morning
  check_email()
  write(report) [2h]

@afternoon
  meeting(standup) [30m]
  code(feature_x) [4h]
```

2. Compile and execute:

```bash
# Compile to markdown (default)
zibox tasks.zbx

# Compile to shell script
zibox tasks.zbx -o shell > schedule.sh

# Compile to JSON
zibox tasks.zbx -o json > schedule.json
```

## DSL Reference

### Blocks

Blocks are defined with `@` followed by a name:

```
@block_name
  task1()
  task2()
```

### Tasks

Tasks are defined with a name and optional parameters:

```
task_name(param1, param2, ...) [duration]
```

### Durations

Durations can be specified in hours (`h`) or minutes (`m`):

- `[2h]` - 2 hours
- `[30m]` - 30 minutes

## Development

### Project Structure

- `src/` - Source code
  - `lib.rs` - Library entry point
  - `main.rs` - CLI entry point
  - `lexer.rs` - Lexer implementation
  - `parser.rs` - Parser implementation
  - `ast.rs` - Abstract Syntax Tree
  - `ir.rs` - Intermediate Representation
  - `error.rs` - Error handling
- `tests/` - Integration tests
- `examples/` - Example .zbx files

### Building

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run with example file
cargo run -- examples/daily.zbx
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
