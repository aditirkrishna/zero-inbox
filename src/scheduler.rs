use crate::ir::{IRProgram, IRTask};
use chrono::{DateTime, Local, Duration, NaiveTime, Timelike};
use log::{info, warn};

pub enum ScheduleMode {
    Naive,
    EarlyBird,
    DeepworkFirst,
}

impl ScheduleMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "naive" => Some(ScheduleMode::Naive),
            "early-bird" | "earlybird" => Some(ScheduleMode::EarlyBird),
            "deepwork" | "deepwork-first" | "deepworkfirst" => Some(ScheduleMode::DeepworkFirst),
            _ => None,
        }
    }
}

pub trait Scheduler {
    fn schedule(&self, program: &mut IRProgram) -> &mut IRProgram;
}

pub struct NaiveScheduler;

impl Scheduler for NaiveScheduler {
    fn schedule(&self, program: &mut IRProgram) -> &mut IRProgram {
        info!("Applying naive scheduling");
        
        // Get today's date with the workday start time
        let now = Local::now();
        let start_time = now.date_naive().and_time(program.metadata.workday_start);
        let end_time = now.date_naive().and_time(program.metadata.workday_end);
        
        let start_datetime = start_time.and_local_timezone(Local).unwrap();
        let end_datetime = end_time.and_local_timezone(Local).unwrap();
        
        let mut current_time = start_datetime;
        
        // Schedule each task sequentially
        for block in &mut program.blocks {
            for task in &mut block.tasks {
                // Check if we're still within working hours
                if current_time >= end_datetime {
                    warn!("Task {} exceeds working hours", task.display_name());
                    continue;
                }
                
                // Schedule the task
                let duration_minutes = task.duration_minutes();
                let end_time = current_time + Duration::minutes(duration_minutes as i64);
                
                task.scheduled_start = Some(current_time);
                task.scheduled_end = Some(end_time);
                
                // Move to the next time slot
                current_time = end_time;
            }
        }
        
        program
    }
}

pub struct EarlyBirdScheduler;

impl Scheduler for EarlyBirdScheduler {
    fn schedule(&self, program: &mut IRProgram) -> &mut IRProgram {
        info!("Applying early bird scheduling (important tasks first)");
        
        // Get today's date with the workday start time
        let now = Local::now();
        let start_time = now.date_naive().and_time(program.metadata.workday_start);
        let end_time = now.date_naive().and_time(program.metadata.workday_end);
        
        let start_datetime = start_time.and_local_timezone(Local).unwrap();
        let end_datetime = end_time.and_local_timezone(Local).unwrap();
        
        let mut current_time = start_datetime;
        
        // Collect all tasks and sort by priority
        let mut all_tasks = Vec::new();
        for block in &program.blocks {
            for task in &block.tasks {
                all_tasks.push(task.clone());
            }
        }
        
        // Sort by priority (high to low)
        all_tasks.sort_by(|a, b| {
            let a_val = priority_to_value(&a.priority);
            let b_val = priority_to_value(&b.priority);
            b_val.cmp(&a_val)
        });
        
        // Schedule tasks in priority order
        for task in &all_tasks {
            // Check if we're still within working hours
            if current_time >= end_datetime {
                warn!("Task {} exceeds working hours", task.display_name());
                continue;
            }
            
            // Schedule the task
            let duration_minutes = task.duration_minutes();
            let end_time = current_time + Duration::minutes(duration_minutes as i64);
            
            // Update the task in the program
            if let Some(program_task) = program.task_map.get_mut(&task.id) {
                program_task.scheduled_start = Some(current_time);
                program_task.scheduled_end = Some(end_time);
            }
            
            // Move to the next time slot
            current_time = end_time;
        }
        
        // Update the blocks with the scheduled tasks
        for block in &mut program.blocks {
            for task in &mut block.tasks {
                if let Some(program_task) = program.task_map.get(&task.id) {
                    task.scheduled_start = program_task.scheduled_start;
                    task.scheduled_end = program_task.scheduled_end;
                }
            }
        }
        
        program
    }
}

pub struct DeepworkScheduler {
    deepwork_tag: String,
}

impl DeepworkScheduler {
    pub fn new(deepwork_tag: String) -> Self {
        Self { deepwork_tag }
    }
}

impl Scheduler for DeepworkScheduler {
    fn schedule(&self, program: &mut IRProgram) -> &mut IRProgram {
        info!("Applying deepwork-first scheduling");
        
        // Get today's date with the workday start time
        let now = Local::now();
        let start_time = now.date_naive().and_time(program.metadata.workday_start);
        let end_time = now.date_naive().and_time(program.metadata.workday_end);
        
        let start_datetime = start_time.and_local_timezone(Local).unwrap();
        let end_datetime = end_time.and_local_timezone(Local).unwrap();
        
        // Calculate the middle of the day for deepwork
        let total_minutes = (end_datetime - start_datetime).num_minutes();
        let deepwork_start = start_datetime + Duration::minutes(total_minutes / 4);
        let deepwork_end = start_datetime + Duration::minutes(3 * total_minutes / 4);
        
        // Collect all tasks
        let mut deepwork_tasks = Vec::new();
        let mut other_tasks = Vec::new();
        
        for block in &program.blocks {
            for task in &block.tasks {
                if task.has_tag(&self.deepwork_tag) {
                    deepwork_tasks.push(task.clone());
                } else {
                    other_tasks.push(task.clone());
                }
            }
        }
        
        // Sort deepwork tasks by priority
        deepwork_tasks.sort_by(|a, b| {
            let a_val = priority_to_value(&a.priority);
            let b_val = priority_to_value(&b.priority);
            b_val.cmp(&a_val)
        });
        
        // Sort other tasks by priority
        other_tasks.sort_by(|a, b| {
            let a_val = priority_to_value(&a.priority);
            let b_val = priority_to_value(&b.priority);
            b_val.cmp(&a_val)
        });
        
        // Schedule deepwork tasks in the middle of the day
        let mut current_time = deepwork_start;
        for task in &deepwork_tasks {
            // Check if we're still within deepwork hours
            if current_time >= deepwork_end {
                warn!("Deepwork task {} exceeds deepwork hours", task.display_name());
                continue;
            }
            
            // Schedule the task
            let duration_minutes = task.duration_minutes();
            let end_time = current_time + Duration::minutes(duration_minutes as i64);
            
            // Update the task in the program
            if let Some(program_task) = program.task_map.get_mut(&task.id) {
                program_task.scheduled_start = Some(current_time);
                program_task.scheduled_end = Some(end_time);
            }
            
            // Move to the next time slot
            current_time = end_time;
        }
        
        // Schedule other tasks before and after deepwork
        let mut current_time = start_datetime;
        for task in &other_tasks {
            // If we've reached the deepwork start time, skip to after deepwork
            if current_time >= deepwork_start && current_time < deepwork_end {
                current_time = deepwork_end;
            }
            
            // Check if we're still within working hours
            if current_time >= end_datetime {
                warn!("Task {} exceeds working hours", task.display_name());
                continue;
            }
            
            // Schedule the task
            let duration_minutes = task.duration_minutes();
            let end_time = current_time + Duration::minutes(duration_minutes as i64);
            
            // Update the task in the program
            if let Some(program_task) = program.task_map.get_mut(&task.id) {
                program_task.scheduled_start = Some(current_time);
                program_task.scheduled_end = Some(end_time);
            }
            
            // Move to the next time slot
            current_time = end_time;
        }
        
        // Update the blocks with the scheduled tasks
        for block in &mut program.blocks {
            for task in &mut block.tasks {
                if let Some(program_task) = program.task_map.get(&task.id) {
                    task.scheduled_start = program_task.scheduled_start;
                    task.scheduled_end = program_task.scheduled_end;
                }
            }
        }
        
        program
    }
}

pub fn create_scheduler(mode: ScheduleMode, deepwork_tag: Option<String>) -> Box<dyn Scheduler> {
    match mode {
        ScheduleMode::Naive => Box::new(NaiveScheduler),
        ScheduleMode::EarlyBird => Box::new(EarlyBirdScheduler),
        ScheduleMode::DeepworkFirst => Box::new(DeepworkScheduler::new(
            deepwork_tag.unwrap_or_else(|| "deepwork".to_string())
        )),
    }
}

fn priority_to_value(priority: &crate::ast::Priority) -> u8 {
    match priority {
        crate::ast::Priority::Low => 0,
        crate::ast::Priority::Medium => 1,
        crate::ast::Priority::High => 2,
        crate::ast::Priority::Critical => 3,
    }
}