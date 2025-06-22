use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::ast::{Block, Program, Task as AstTask, TaskDuration as AstDuration};
use crate::error::Result;

/// Represents a scheduled task in the IR
#[derive(Debug, Clone, Serialize)]
pub struct Task {
    /// The original task name
    pub name: String,
    
    /// Task parameters
    pub params: Vec<String>,
    
    /// Estimated duration of the task
    pub duration: Option<Duration>,
    
    /// Scheduled start time (if known)
    pub start_time: Option<DateTime<Local>>,
    
    /// Scheduled end time (if known)
    pub end_time: Option<DateTime<Local>>,
    
    /// Block this task belongs to
    pub block: String,
    
    /// Any tags associated with the task
    pub tags: Vec<String>,
    
    /// Priority (higher = more important)
    pub priority: u8,
}

/// Represents a scheduled block in the IR
#[derive(Debug, Clone, Serialize)]
pub struct IRBlock {
    /// The original block name
    pub name: String,
    
    /// Scheduled tasks in this block
    pub tasks: Vec<Task>,
    
    /// Scheduled start time of the block
    pub start_time: Option<DateTime<Local>>,
    
    /// Scheduled end time of the block
    pub end_time: Option<DateTime<Local>>,
}

/// The complete IR representation of a Zero Inbox program
#[derive(Debug, Default, Serialize)]
pub struct IRProgram {
    /// All blocks in the program
    pub blocks: Vec<IRBlock>,
    
    /// Total estimated duration of all tasks
    pub total_duration: Duration,
    
    /// Metadata about the program
    pub metadata: HashMap<String, String>,
}

impl IRProgram {
    /// Create a new, empty IR program
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a block to the program
    pub fn add_block(&mut self, block: IRBlock) {
        self.blocks.push(block);
    }
}

/// Convert an AST Program to an IR Program
pub fn lower(program: Program) -> Result<IRProgram> {
    let mut ir_program = IRProgram::new();
    
    // Convert each block
    for block in program.blocks {
        let mut ir_block = IRBlock {
            name: block.name.clone(),
            tasks: Vec::new(),
            start_time: None,
            end_time: None,
        };
        
        // Convert each task
        for task in block.tasks {
            let ir_task = Task {
                name: task.name,
                params: task.params,
                duration: task.duration.map(|d| d.to_std()),
                start_time: None,
                end_time: None,
                block: block.name.clone(),
                tags: Vec::new(),
                priority: 0, // Default priority
            };
            
            ir_block.tasks.push(ir_task);
        }
        
        ir_program.add_block(ir_block);
    }
    
    // Calculate total duration
    ir_program.total_duration = ir_program.blocks.iter()
        .flat_map(|b| &b.tasks)
        .filter_map(|t| t.duration)
        .sum();
    
    Ok(ir_program)
}

/// Optimize the IR program
pub fn optimize(program: &mut IRProgram) {
    // Simple optimization: sort tasks by duration (longest first)
    for block in &mut program.blocks {
        block.tasks.sort_by(|a, b| {
            let a_dur = a.duration.unwrap_or_default();
            let b_dur = b.duration.unwrap_or_default();
            b_dur.cmp(&a_dur) // Reverse order (longest first)
        });
    }
    
    // TODO: More sophisticated optimizations
    // - Merge similar tasks
    // - Reorder based on dependencies
    // - Allocate time slots
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Block, Program, Task};
    
    #[test]
    fn test_lower_program() {
        let mut program = Program::new();
        
        let mut block1 = Block::new("morning");
        block1.add_task(Task {
            name: "check_email".to_string(),
            params: vec!["inbox".to_string()],
            duration: None,
        });
        
        let mut block2 = Block::new("afternoon");
        block2.add_task(Task {
            name: "write".to_string(),
            params: vec!["report".to_string()],
            duration: None,
        });
        
        program.add_block(block1);
        program.add_block(block2);
        
        let ir_program = lower(program).unwrap();
        
        assert_eq!(ir_program.blocks.len(), 2);
        assert_eq!(ir_program.blocks[0].name, "morning");
        assert_eq!(ir_program.blocks[0].tasks[0].name, "check_email");
        assert_eq!(ir_program.blocks[1].name, "afternoon");
        assert_eq!(ir_program.blocks[1].tasks[0].name, "write");
    }
    
    #[test]
    fn test_optimize() {
        let mut program = IRProgram::new();
        
        let mut block = IRBlock {
            name: "test".to_string(),
            tasks: vec![
                Task {
                    name: "short".to_string(),
                    params: vec![],
                    duration: Some(Duration::from_secs(300)), // 5 min
                    start_time: None,
                    end_time: None,
                    block: "test".to_string(),
                    tags: vec![],
                    priority: 0,
                },
                Task {
                    name: "long".to_string(),
                    params: vec![],
                    duration: Some(Duration::from_secs(3600)), // 1 hour
                    start_time: None,
                    end_time: None,
                    block: "test".to_string(),
                    tags: vec![],
                    priority: 0,
                },
            ],
            start_time: None,
            end_time: None,
        };
        
        program.add_block(block);
        
        // Before optimization: short task first
        assert_eq!(program.blocks[0].tasks[0].name, "short");
        
        optimize(&mut program);
        
        // After optimization: long task first
        assert_eq!(program.blocks[0].tasks[0].name, "long");
    }
}
