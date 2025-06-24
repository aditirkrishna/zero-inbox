use crate::ir::{IRProgram, IRTask};
use chrono::{Local, Duration};
use std::io::{self, Write};
use std::thread;
use std::fs::OpenOptions;
use anyhow::{Result, anyhow};
use colored::Colorize;
use log::{info, warn};

pub fn execute(program: &mut IRProgram, dry_run: bool) -> Result<()> {
    println!("{}", "Starting Zero Inbox execution".green().bold());
    println!("Press 'q' to quit, 's' to skip a task, or Enter to continue");
    println!();
    
    // Open logbook
    let mut logbook = OpenOptions::new()
        .create(true)
        .append(true)
        .open("zibox.log")?;
    
    // Log execution start
    writeln!(
        &mut logbook,
        "[{}] Starting execution with {} blocks and {} tasks",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        program.blocks.len(),
        program.all_tasks().len()
    )?;
    
    // Get all scheduled tasks sorted by start time
    let mut tasks: Vec<_> = program.all_tasks()
        .into_iter()
        .filter(|t| t.scheduled_start.is_some())
        .collect();
    
    tasks.sort_by_key(|t| t.scheduled_start);
    
    for task in &tasks {
        let task_name = task.display_name();
        let duration = task.duration_minutes();
        
        // Get the task from the program so we can modify it
        let program_task = match program.task_map.get_mut(&task.id) {
            Some(t) => t,
            None => continue,
        };
        
        println!("{} {} ({})", "Starting:".blue().bold(), task_name, format_duration(duration));
        
        // Log task start
        writeln!(
            &mut logbook,
            "[{}] Starting task: {} ({})",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            task_name,
            format_duration(duration)
        )?;
        
        // Ask for confirmation
        print!("Press Enter to start, 's' to skip, or 'q' to quit: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "q" => {
                println!("{}", "Execution aborted".red().bold());
                writeln!(
                    &mut logbook,
                    "[{}] Execution aborted by user",
                    Local::now().format("%Y-%m-%d %H:%M:%S")
                )?;
                return Ok(());
            },
            "s" => {
                println!("{} {}", "Skipping:".yellow().bold(), task_name);
                writeln!(
                    &mut logbook,
                    "[{}] Skipped task: {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    task_name
                )?;
                continue;
            },
            _ => {}
        }
        
        // Execute the task (simulate with sleep)
        if duration > 0 && !dry_run {
            let start_time = Local::now();
            let end_time = start_time + Duration::minutes(duration as i64);
            
            println!(
                "Working on task until {} (press Ctrl+C to interrupt)",
                end_time.format("%H:%M:%S")
            );
            
            // Sleep in small increments to allow for interruption
            let total_seconds = duration * 60;
            let increment = 5; // 5 seconds
            
            for i in 0..total_seconds / increment {
                thread::sleep(std::time::Duration::from_secs(increment));
                
                // Show progress
                let progress = (i + 1) as f64 / (total_seconds / increment) as f64;
                print!("\rProgress: [");
                let width = 30;
                let filled = (progress * width as f64) as usize;
                
                for j in 0..width {
                    if j < filled {
                        print!("=");
                    } else {
                        print!(" ");
                    }
                }
                
                print!("] {:.0}%", progress * 100.0);
                io::stdout().flush()?;
            }
            
            println!("\n{} {}", "Completed:".green().bold(), task_name);
        } else {
            if dry_run {
                println!("{} {} (dry run)", "Simulating:".yellow().bold(), task_name);
                thread::sleep(std::time::Duration::from_secs(1));
            }
            println!("{} {}", "Completed:".green().bold(), task_name);
        }
        
        // Mark task as completed
        program_task.completed = true;
        
        // Log task completion
        writeln!(
            &mut logbook,
            "[{}] Completed task: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            task_name
        )?;
        
        println!();
    }
    
    println!("{}", "All tasks completed!".green().bold());
    
    // Log execution end
    writeln!(
        &mut logbook,
        "[{}] Execution completed",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    
    Ok(())
}

fn format_duration(minutes: u64) -> String {
    if minutes < 60 {
        format!("{}m", minutes)
    } else if minutes % 60 == 0 {
        format!("{}h", minutes / 60)
    } else {
        format!("{}h {}m", minutes / 60, minutes % 60)
    }
}