# Zero Inbox

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/zero-inbox/zero-inbox/workflows/Rust/badge.svg)](https://github.com/zero-inbox/zero-inbox/actions)

A minimalist attention compiler for structured daily execution.

## Overview

Zero Inbox is a DSL compiler for attention/task modeling. It turns structured text files into optimized execution plans.

The project transforms human-readable task definitions into structured, optimized, and executable plans. Built in Rust, designed for precision, minimalism, and deep dev learning.

## Installation

```bash
# Install from crates.io
cargo install zibox

# Or build from source
git clone https://github.com/zero-inbox/zero-inbox.git
cd zero-inbox
cargo install --path .
```

## Usage

Create a `.zbx` file with your tasks:

```
@morning
  write(report) [2h]
  clear(inbox)

drink(coffee) [15m]

@evening
  reflect(day) [1h]
  read(book) [30m]
```

Then compile it:

```bash
zibox daily_plan.zbx
```

### Output Formats

Zero Inbox supports multiple output formats:

```bash
# Generate a shell script
zibox daily_plan.zbx --output-format shell --output-file schedule.sh

# Generate markdown
zibox daily_plan.zbx --output-format markdown

# Generate JSON
zibox daily_plan.zbx --output-format json

# Generate calendar file
zibox daily_plan.zbx --calendar
```

### Scheduling Options

```bash
# Set work hours
zibox daily_plan.zbx --workday-start 09:00 --workday-end 18:00

# Focus on specific tags
zibox daily_plan.zbx --focus-tag #deepwork

# Visualize schedule
zibox daily_plan.zbx --visualize-schedule
```

## Features

- Parse `.zbx` files with a clean, minimal syntax
- Generate executable shell scripts
- Create markdown task lists
- Export to calendar formats
- Optimize task scheduling
- Support for task dependencies
- Visualization options

## License

This project is licensed under the MIT License - see the LICENSE file for details.