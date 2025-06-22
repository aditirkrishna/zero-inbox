use std::time::Duration;
use std::fmt;

/// Represents a duration in the Zero Inbox DSL
#[derive(Debug, Clone, PartialEq)]
pub struct TaskDuration {
    minutes: u32,
}

impl TaskDuration {
    /// Create a new duration from a string like "2h" or "30m"
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim_matches(|c| c == '[' || c == ']');
        
        if s.ends_with('h') {
            let hours = s[..s.len()-1].parse::<u32>().ok()?;
            Some(Self {
                minutes: hours * 60,
            })
        } else if s.ends_with('m') {
            let minutes = s[..s.len()-1].parse::<u32>().ok()?;
            Some(Self { minutes })
        } else {
            None
        }
    }
    
    /// Convert to std::time::Duration
    pub fn to_std(&self) -> Duration {
        Duration::from_secs(u64::from(self.minutes) * 60)
    }
}

impl fmt::Display for TaskDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.minutes % 60 == 0 {
            write!(f, "{}h", self.minutes / 60)
        } else {
            write!(f, "{}m", self.minutes)
        }
    }
}

/// Represents a task in the Zero Inbox DSL
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub params: Vec<String>,
    pub duration: Option<TaskDuration>,
}

impl Task {
    /// Create a new task from a string like "task_name(param1, param2)"
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();
        if let Some((name, params_str)) = s.split_once('(') {
            let name = name.trim().to_string();
            let params_str = params_str.trim_end_matches(')');
            let params = if params_str.is_empty() {
                Vec::new()
            } else {
                params_str.split(',').map(|s| s.trim().to_string()).collect()
            };
            
            Some(Self {
                name,
                params,
                duration: None,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        write!(f, "{}", self.params.join(", "))?;
        write!(f, ")")
    }
}

/// Represents a block of tasks in the Zero Inbox DSL
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Block {
    /// Create a new block with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tasks: Vec::new(),
        }
    }
    
    /// Add a task to this block
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

/// Represents a complete Zero Inbox program
#[derive(Debug, Default, PartialEq)]
pub struct Program {
    pub blocks: Vec<Block>,
}

impl Program {
    /// Create a new, empty program
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a block to the program
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_parsing() {
        let task = Task::from_str("write(report, urgent)").unwrap();
        assert_eq!(task.name, "write");
        assert_eq!(task.params, vec!["report", "urgent"]);
        assert!(task.duration.is_none());
    }
    
    #[test]
    fn test_duration_parsing() {
        let duration = TaskDuration::from_str("[2h]").unwrap();
        assert_eq!(duration.minutes, 120);
        assert_eq!(duration.to_string(), "2h");
        
        let duration = TaskDuration::from_str("[30m]").unwrap();
        assert_eq!(duration.minutes, 30);
        assert_eq!(duration.to_string(), "30m");
    }
    
    #[test]
    fn test_block_creation() {
        let mut block = Block::new("morning");
        block.add_task(Task::from_str("check_email()").unwrap());
        block.add_task(Task::from_str("write(report)").unwrap());
        
        assert_eq!(block.name, "morning");
        assert_eq!(block.tasks.len(), 2);
        assert_eq!(block.tasks[0].name, "check_email");
        assert_eq!(block.tasks[1].name, "write");
    }
}
