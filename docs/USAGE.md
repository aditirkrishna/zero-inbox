# zero-inbox: Usage Guide

>Usage Guide v1.0 - Because reading manuals shouldn't be painful

Welcome to the zero-inbox CLI. This guide explains how to turn your caffeine-fueled thoughts into structured productivity.

## Writing .zbx Files

Zero Inbox uses a simple, human-readable syntax that even your sleep-deprived brain can understand. Here's a realistic example:

```
@morning
  check(email) [15m]  # Mostly spam anyway
  standup(meeting) [30m]  # The daily ritual

@afternoon
  code(feature) [2h]  # Optimistic estimate
  meeting(planning) [1h]  # Could've been an email
```

### Syntax Elements

1. **Blocks**: Defined with `@` prefix, like `@morning` or `@work`.
2. **Tasks**: Written as `task_name(parameters)`.
3. **Duration**: Added in square brackets, like `[30m]` or `[2h]`.
4. **Tags**: Added with `#` prefix, like `#deepwork` or `#admin`.
5. **Priority**: Added with `p:` prefix, like `p:high` or `p:low`.
6. **Dependencies**: Added with `after:` prefix, like `after:task1,task2`.

### Complete Example

```
@morning
  check(email) [15m] #admin p:medium
  standup(meeting) [30m] #collaboration p:high
  code(feature) [2h] #deepwork p:critical after:standup

@afternoon
  review(pull-requests) [1h] #collaboration p:high
  debug(issue) [1h30m] #deepwork p:high
  meeting(planning) [1h] #collaboration p:medium
```

## Compiling .zbx Files

Use the `zibox` command to compile your .zbx files:

```bash
# Basic compilation (outputs markdown to stdout)
zibox my_plan.zbx

# Specify output format
zibox my_plan.zbx --output-format shell

# Save to file
zibox my_plan.zbx --output-format markdown --output-file my_plan.md

# Generate calendar file
zibox my_plan.zbx --output-format calendar --output-file my_plan.ics
```

## Scheduling Options

Zero Inbox can schedule your tasks based on different strategies:

```bash
# Set work hours
zibox my_plan.zbx --workday-start 09:00 --workday-end 17:00

# Choose scheduling mode
zibox my_plan.zbx --schedule-mode early-bird
zibox my_plan.zbx --schedule-mode deepwork-first

# Focus on specific tags
zibox my_plan.zbx --focus-tag deepwork --focus-tag admin

# Set optimization level
zibox my_plan.zbx --opt-level 2
```

## Visualization and Execution

```bash
# Visualize the schedule
zibox my_plan.zbx --visualize-schedule

# Show intermediate representation
zibox my_plan.zbx --show-ir

# Run the plan
zibox my_plan.zbx --run

# Dry run (simulate execution)
zibox my_plan.zbx --run --dry-run
```

## Configuration File

You can create a `.ziboxrc` file in your home directory or project directory to set default options:

```toml
output_format = "markdown"
workday_start = "09:00"
workday_end = "17:00"
schedule_mode = "deepwork-first"
optimization_level = 2
focus_tags = ["deepwork", "admin"]
max_parallel = 1
deepwork_tag = "deepwork"
```

## Creating New Files

You can create a new .zbx file from a template:

```bash
zibox new my_plan
```

## Listing Supported Formats

To see all supported output formats:

```bash
zibox formats
```

<p align="center">built with ❤️ by aditi ramakrishnan</p>

*Usage Guide v1.0 - Because reading manuals shouldn't be painful*