use clap::{Parser, Subcommand, Args};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use colored::Colorize;
use log::{info, warn, error};
use env_logger::Env;

use zero_inbox::{
    config::Config,
    codegen::{OutputFormat, visualize_schedule},
    runtime,
    utils::{sanitize_output_name, ensure_dir_exists},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser)]
#[command(
    name = "zibox",
    version = VERSION,
    author = AUTHOR,
    about = DESCRIPTION,
    long_about = "Zero Inbox is a DSL compiler for attention/task modeling. It turns structured text files into optimized execution plans."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Input .zbx file to compile
    #[arg(value_name = "FILE")]
    input_file: Option<PathBuf>,
    
    /// Output format (shell, markdown, json, calendar)
    #[arg(short, long, value_name = "FORMAT")]
    output_format: Option<String>,
    
    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    output_file: Option<PathBuf>,
    
    /// Workday start time (HH:MM)
    #[arg(long, value_name = "TIME")]
    workday_start: Option<String>,
    
    /// Workday end time (HH:MM)
    #[arg(long, value_name = "TIME")]
    workday_end: Option<String>,
    
    /// Schedule mode (naive, early-bird, deepwork-first)
    #[arg(long, value_name = "MODE")]
    schedule_mode: Option<String>,
    
    /// Optimization level (0-3)
    #[arg(short = 'O', long, value_name = "LEVEL")]
    opt_level: Option<u8>,
    
    /// Focus on specific tags
    #[arg(long, value_name = "TAG")]
    focus_tag: Option<Vec<String>>,
    
    /// Maximum number of parallel tasks
    #[arg(long, value_name = "NUM")]
    max_parallel: Option<usize>,
    
    /// Tag to use for deepwork scheduling
    #[arg(long, value_name = "TAG")]
    deepwork_tag: Option<String>,
    
    /// Dry run (don't execute tasks)
    #[arg(long)]
    dry_run: bool,
    
    /// Show intermediate representation
    #[arg(long)]
    show_ir: bool,
    
    /// Visualize schedule
    #[arg(long)]
    visualize_schedule: bool,
    
    /// Run the compiled plan
    #[arg(long)]
    run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List supported output formats with examples
    Formats,
    
    /// Create a new .zbx file from a template
    New {
        /// Name of the new file
        #[arg(value_name = "NAME")]
        name: String,
    },
}

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Load config
    let mut config = Config::load()?;
    
    // Override config with command line arguments
    if let Some(format) = cli.output_format {
        config.output_format = format;
    }
    
    if let Some(file) = &cli.output_file {
        config.output_file = Some(file.to_string_lossy().to_string());
    }
    
    if let Some(start) = cli.workday_start {
        config.workday_start = start;
    }
    
    if let Some(end) = cli.workday_end {
        config.workday_end = end;
    }
    
    if let Some(mode) = cli.schedule_mode {
        config.schedule_mode = mode;
    }
    
    if let Some(level) = cli.opt_level {
        config.optimization_level = level;
    }
    
    if let Some(tags) = cli.focus_tag {
        config.focus_tags = tags;
    }
    
    if let Some(parallel) = cli.max_parallel {
        config.max_parallel = parallel;
    }
    
    if let Some(tag) = cli.deepwork_tag {
        config.deepwork_tag = tag;
    }
    
    config.dry_run = cli.dry_run;
    config.show_ir = cli.show_ir;
    config.visualize_schedule = cli.visualize_schedule;
    
    // Handle subcommands
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Formats => {
                return show_formats();
            },
            Commands::New { name } => {
                return create_new_file(&name);
            },
        }
    }
    
    // Handle main command
    let input_file = match cli.input_file {
        Some(path) => path,
        None => {
            eprintln!("{}", "Error: No input file specified".red().bold());
            eprintln!("Run 'zibox --help' for usage information");
            return Err(anyhow!("No input file specified"));
        }
    };
    
    // Check if input file exists
    if !input_file.exists() {
        return Err(anyhow!("Input file not found: {}", input_file.display()));
    }
    
    // Compile the input file
    info!("Compiling {}", input_file.display());
    let output = zero_inbox::compile(&input_file, &config)?;
    
    // Show IR if requested
    if config.show_ir {
        let ast = zero_inbox::parse_file(&input_file)?;
        let metadata = config.to_ir_metadata()?;
        let ir = zero_inbox::ir::to_ir(&ast, metadata);
        
        println!("\n{}\n", "Intermediate Representation:".yellow().bold());
        println!("{:#?}", ir);
        println!();
    }
    
    // Visualize schedule if requested
    if config.visualize_schedule {
        let ast = zero_inbox::parse_file(&input_file)?;
        let metadata = config.to_ir_metadata()?;
        let mut ir = zero_inbox::ir::to_ir(&ast, metadata);
        
        // Optimize and schedule
        zero_inbox::optimizer::optimize(&mut ir);
        let schedule_mode = config.get_schedule_mode()?;
        let scheduler = zero_inbox::scheduler::create_scheduler(
            schedule_mode,
            if config.deepwork_tag.is_empty() { None } else { Some(config.deepwork_tag.clone()) }
        );
        scheduler.schedule(&mut ir);
        
        println!("\n{}\n", "Schedule Visualization:".yellow().bold());
        let visualization = visualize_schedule(&ir)?;
        println!("{}", visualization);
        println!();
    }
    
    // Write output to file or print to stdout
    if let Some(output_path) = &config.output_file {
        let path = Path::new(output_path);
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            ensure_dir_exists(parent)?;
        }
        
        fs::write(path, &output)
            .map_err(|e| anyhow!("Failed to write output file: {}", e))?;
        
        println!("Output written to {}", path.display());
    } else {
        println!("\n{}\n", output);
    }
    
    // Run the plan if requested
    if cli.run {
        let ast = zero_inbox::parse_file(&input_file)?;
        let metadata = config.to_ir_metadata()?;
        let mut ir = zero_inbox::ir::to_ir(&ast, metadata);
        
        // Optimize and schedule
        zero_inbox::optimizer::optimize(&mut ir);
        let schedule_mode = config.get_schedule_mode()?;
        let scheduler = zero_inbox::scheduler::create_scheduler(
            schedule_mode,
            if config.deepwork_tag.is_empty() { None } else { Some(config.deepwork_tag.clone()) }
        );
        scheduler.schedule(&mut ir);
        
        // Execute the plan
        runtime::execute(&mut ir, config.dry_run)?;
    }
    
    Ok(())
}

fn show_formats() -> Result<()> {
    println!("{}", "Supported Output Formats:".green().bold());
    println!("======================");
    
    println!("\n{}", "shell (.sh)".yellow().bold());
    println!("  Generates a shell script that can be executed to run your tasks.");
    println!("  Example: zibox plan.zbx --output-format shell --output-file plan.sh");
    
    println!("\n{}", "markdown (.md)".yellow().bold());
    println!("  Generates a markdown document with your tasks formatted as a checklist.");
    println!("  Example: zibox plan.zbx --output-format markdown --output-file plan.md");
    
    println!("\n{}", "json (.json)".yellow().bold());
    println!("  Generates a JSON representation of your tasks for integration with other tools.");
    println!("  Example: zibox plan.zbx --output-format json --output-file plan.json");
    
    println!("\n{}", "calendar (.ics)".yellow().bold());
    println!("  Generates an iCalendar file that can be imported into calendar applications.");
    println!("  Example: zibox plan.zbx --output-format calendar --output-file plan.ics");
    
    Ok(())
}

fn create_new_file(name: &str) -> Result<()> {
    let filename = sanitize_output_name(name, "zbx");
    let path = Path::new(&filename);
    
    // Check if file already exists
    if path.exists() {
        return Err(anyhow!("File already exists: {}", path.display()));
    }
    
    // Create template content
    let template = r#"@morning
  review(inbox) [30m] #admin p:high
  write(report) [2h] #deepwork p:critical

@afternoon
  meeting(team) [1h] #collaboration
  code(feature) [3h] #deepwork p:high after:meeting

@evening
  exercise(run) [45m] #health
  read(book) [30m] #learning
"#;
    
    // Write template to file
    fs::write(path, template)
        .map_err(|e| anyhow!("Failed to create file: {}", e))?;
    
    println!("Created new .zbx file: {}", path.display());
    
    Ok(())
}
