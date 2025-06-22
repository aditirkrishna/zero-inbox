**Zero Inbox: 200-Step Plan for a Minimalist Attention Compiler (Rust CLI Project)**

> A focused, compiler-style system for transforming human-readable task definitions into structured, optimized, and executable plans. Built in Rust, designed for precision, minimalism, and deep dev learning.

---

**PHASE 1: Foundation & Setup (Steps 1–20)**

**1.** Initialize a new Rust project using `cargo new zero_inbox`. Set up standard `src/`, `tests/`, and `Cargo.toml` file. This is your working foundation.

**2.** Choose your CLI framework (suggested: `clap`) and add it as a dependency. Set up a `main.rs` that takes a `.zbx` file as input and prints "compiling..." to test wiring.

**3.** Define a minimalist `.zbx` DSL format. For example:

```
@morning
  write(report) [2h]
  clear(inbox)
```

This is your domain language. It should be readable, indent-based or syntax-light.

**4.** Write a README with the project’s scope: "Zero Inbox is a DSL compiler for attention/task modeling. It turns structured text files into optimized execution plans."

**5.** Create a basic lexer using the `logos` crate or manual string splitting. Tokenize `@`, `task_name`, `duration`, and `block` symbols.

**6.** Write simple tests for the lexer. Provide inputs like `@night clear(inbox)` and ensure correct tokens are generated.

**7.** Create a struct `Token` in `token.rs` that includes variant types like `Header`, `Task`, `Duration`, `EOL`, etc. Keep it minimal.

**8.** Design the `parser.rs` file. It will consume the `Vec<Token>` and return a custom `Program` struct made of `Block` and `Task` items.

**9.** Define structs `Program`, `Block`, and `Task`. Example:

```rust
struct Task { name: String, duration: Option<Duration> }
struct Block { label: String, tasks: Vec<Task> }
struct Program { blocks: Vec<Block> }
```

Keep everything pure and testable.

**10.** Write a hand-rolled parser that builds `Program` from `Vec<Token>`. Include error handling for missing blocks or malformed lines.

**11.** Write integration tests for parser: feed a real `.zbx` file and verify that the AST (`Program`) is constructed correctly.

**12.** Start a `lib.rs` that re-exports modules: `mod lexer; mod parser; mod ir; mod codegen;`. Your crate will be used later in CLI.

**13.** Write a function `parse_file(path: &str) -> Result<Program>` that glues lexer + parser. This is your end-to-end frontend.

**14.** Implement a `Duration` struct with parsing support: `2h`, `30m`, etc. Make sure it uses `chrono` or basic custom units.

**15.** Design an internal IR structure that closely resembles the `Program` but adds optimization metadata (start\_time, priority, tags).

**16.** Create a new file `ir.rs`. Define structs like `IRBlock`, `IRTask` with fields like `estimated_time`, `tag`, `exec_order`.

**17.** Write a lowering function: `fn lower(program: Program) -> IRPlan`. This turns parser output into IR with default timings.

**18.** In `optimizer.rs`, write a simple pass that reorders tasks by duration or priority. Don’t overdo — just enough to show proof.

**19.** Add logging via `tracing` or `println!` that shows phase progress: "Parsed 3 blocks", "Optimized 6 tasks".

**20.** Update `main.rs` to call `parse_file`, `lower`, and `optimize`. Print the IR as a tree to confirm the pipeline works end-to-end.

---

**PHASE 2: Code Generation & Output (Steps 21–40)**

**21.** In `codegen.rs`, create a function `emit_shell_script(ir: &IRPlan) -> String` that turns IR tasks into basic shell commands with `echo`, `sleep`, or `notify-send`.

**22.** Create a corresponding function `emit_markdown(ir: &IRPlan) -> String` that generates a nicely formatted `.md` version of the task plan with sections and durations.

**23.** Add a new CLI flag `--output-format` with choices: `shell`, `markdown`, `json`. Default to markdown if not specified.

**24.** Write the `emit_json(ir: &IRPlan)` function using `serde_json`. Each `IRBlock` becomes a JSON object with nested tasks.

**25.** Create a directory `examples/` and write 2-3 `.zbx` task plans to test compilation output end-to-end across formats.

**26.** Add a utility `utils.rs` that includes helpers like `slugify`, `format_duration`, and `sanitize_output_name`.

**27.** Support a flag `--output-file <path>` that writes generated output (e.g., `schedule.sh`) instead of printing to stdout.

**28.** Improve the shell codegen with timestamps using `date +%H:%M`, echoing task start times, and delays (`sleep 300` for 5 minutes).

**29.** Add environment detection: if `notify-send` or `say` exists, emit audio/desktop notifications in the shell output optionally.

**30.** Integrate a timestamp-aware `Schedule` struct in the IR layer to support time-based sequencing (start\_at, end\_at).

**31.** Write `schedule_optimizer.rs` that scans for overlapping or inefficient durations and warns in the output.

**32.** Add color-coded CLI output using `colored` crate. Highlight phase status, task block headers, durations.

**33.** Enable `--dry-run` flag that simulates schedule with delays and echoing instead of running real commands.

**34.** Write tests for each codegen format: shell, markdown, and JSON. Compare against static golden files in `/tests/golden/`.

**35.** Provide an `--opt-level` flag like compilers (0 = raw, 1 = deduped, 2 = reordered by priority).

**36.** Set up `.ziboxrc` config file support. Let users define default format, timezone, verbosity in a TOML file.

**37.** Enable CLI command `zibox formats` to list supported output formats with samples.

**38.** Refactor IR to support `tagged` tasks — allow tags like `#deepwork`, `#admin` for later filtering and prioritization.

**39.** Support tag-based optimization: let `--focus-tag #deepwork` compile only relevant blocks into final output.

**40.** Add a debug flag `--show-ir` that prints the intermediate representation before codegen for inspection.

PHASE 3: Scheduling Intelligence & Execution Layer (Steps 41–60)

41. Design a Scheduler trait in scheduler.rs that defines an interface for task ordering: fn schedule(&self, plan: &IRPlan) -> Schedule.

42. Implement a default NaiveScheduler that simply queues tasks in block order with no overlaps and fixed start times based on durations.

43. Extend IRTask to include scheduled_start: Option<chrono::DateTime> and scheduled_end. Set by the scheduler after planning.

44. Write a fn resolve_schedule(plan: &IRPlan) -> Schedule function that assigns start/end times using a rolling window.

45. Introduce a --schedule-mode CLI flag to switch between naive, early-bird, or deepwork-priority strategies.

46. Add PriorityScheduler that reorders tasks based on tags like #deepwork or explicit priority: 1, etc., pushing high-priority tasks earlier in the day.

47. Implement simple work-hour bounds: --workday-start 09:00 and --workday-end 18:00 to constrain scheduling range.

48. Extend IRPlan to allow metadata at the plan level (like timezone, workday span, preferred slots).

49. Add test cases in tests/scheduling.rs to check that scheduled plans respect durations, order, and time windows.

50. Create a time_utils.rs with helpers like round_to_nearest_15_min() and add_duration(start, duration).

51. Add CLI option --calendar that generates .ics files using the icalendar crate. Export each task as a calendar event.

52. Emit a warning when task durations exceed the available time window (conflicts or overflows).

53. Implement --max-parallel <N> option to allow scheduling tasks in parallel if no dependencies or blocks require sequencing.

54. Introduce task dependencies: allow syntax like send(email) after draft(email) and update IR to reflect dependencies.

55. Write a topological sort on the IR graph to enforce dependency resolution in scheduling.

56. Add --visualize-schedule option that generates a Gantt chart using ASCII output or .svg (optionally use plotters crate).

57. Write a runtime.rs module that can execute a compiled plan — at minimum, a loop that runs shell commands or simulates delays.

58. Implement a --run flag that invokes runtime::execute(plan) to step through the compiled tasks with optional confirmation.

59. Add support for task cancellation or skips during execution by listening to user input (e.g., press s to skip).

60. Build a logbook.txt or zibox.log file that appends timestamps and outcomes of executed tasks — useful for history and debugging.

PHASE 4: Polish & Finalization (Steps 61–75)

61. Refactor the project structure for clarity: separate frontend/, backend/, scheduler/, and output/ modules. Move relevant files to each.

62. Create a config.rs that manages CLI options and config file loading (.ziboxrc). Centralize default values here.

63. Add detailed error handling and friendly messages using anyhow or thiserror to replace panics and unwraps.

64. Improve CLI experience with --help docs, usage examples, and descriptions for all options. Test zibox --help for polish.

65. Add --version and --about metadata using clap macros to make the binary professional-grade.

66. Run cargo fmt, clippy, and cargo audit to clean up the codebase and ensure everything is idiomatic, safe, and secure.

67. Write a simple guide docs/USAGE.md showing how to write .zbx files and run the compiler step-by-step.

68. Create a Makefile or justfile with common commands: build, test, run-example, gen-docs, fmt, etc.

69. Package the project using cargo install --path . and test install from scratch. Ensure binaries work standalone.

70. Write a changelog CHANGELOG.md summarizing major phases. Start versioning from v0.1.0.

71. Add automated tests for CLI flags and error conditions using assert_cmd and tempfile crates.

72. Set up GitHub Actions CI to run on push and pull_request with checks: build, test, lint, fmt.

73. Add a LICENSE file (recommendation: MIT or Apache 2.0) and a clean README.md badge setup.

74. Build an example .zbx plan file and a corresponding output/ folder with compiled .sh, .md, and .ics files for demo.

75. Tag a v0.1.0 release, publish the project repo, and optionally submit to crates.io as zibox, with installation steps and roadmap.

PROJECT COMPLETE. STOP HERE.

You now have a full minimalist compiler project in Rust:

Real DSL, real compiler pipeline

IR, scheduler, runtime

Clean CLI, helpful flags

Executable output with .sh, .md, .ics

Docs, tests, versioned, and ready for the world

No fluff. No overengineering. Just raw, clean, dev-core craftsmanship.