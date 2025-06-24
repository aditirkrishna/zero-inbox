use std::str::FromStr;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskDuration {
    pub minutes: u64,
}

impl TaskDuration {
    pub fn from_minutes(mins: u64) -> Self {
        Self { minutes: mins }
    }
    
    pub fn to_human_string(&self) -> String {
        if self.minutes < 60 {
            format!("{}m", self.minutes)
        } else if self.minutes % 60 == 0 {
            format!("{}h", self.minutes / 60)
        } else {
            format!("{}h {}m", self.minutes / 60, self.minutes % 60)
        }
    }
}

impl FromStr for TaskDuration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        if let Some(stripped) = s.strip_suffix("h") {
            if let Ok(hours) = stripped.parse::<u64>() {
                return Ok(Self::from_minutes(hours * 60));
            }
        } else if let Some(stripped) = s.strip_suffix("m") {
            if let Ok(mins) = stripped.parse::<u64>() {
                return Ok(Self::from_minutes(mins));
            }
        }
        Err(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "low" | "1" => Ok(Priority::Low),
            "medium" | "2" => Ok(Priority::Medium),
            "high" | "3" => Ok(Priority::High),
            "critical" | "4" => Ok(Priority::Critical),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub params: Vec<String>,
    pub duration: Option<TaskDuration>,
    pub tags: HashSet<String>,
    pub priority: Priority,
    pub depends_on: Vec<String>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            duration: None,
            tags: HashSet::new(),
            priority: Priority::default(),
            depends_on: Vec::new(),
        }
    }
    
    pub fn with_duration(mut self, duration: Option<TaskDuration>) -> Self {
        self.duration = duration;
        self
    }
    
    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.params = params;
        self
    }
    
    pub fn with_tags(mut self, tags: HashSet<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_dependencies(mut self, depends_on: Vec<String>) -> Self {
        self.depends_on = depends_on;
        self
    }
    
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Block {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tasks: Vec::new(),
        }
    }
    
    pub fn with_tasks(mut self, tasks: Vec<Task>) -> Self {
        self.tasks = tasks;
        self
    }
    
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
