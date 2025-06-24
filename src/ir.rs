use crate::ast::{Block as ASTBlock, Task as ASTTask, TaskDuration, Priority};
use chrono::{DateTime, Local, Duration, NaiveTime};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRTask {
    pub id: String,
    pub name: String,
    pub params: Vec<String>,
    pub duration: Option<TaskDuration>,
    pub block: String,
    pub tags: HashSet<String>,
    pub priority: Priority,
    pub depends_on: Vec<String>,
    pub scheduled_start: Option<DateTime<Local>>,
    pub scheduled_end: Option<DateTime<Local>>,
    pub completed: bool,
}

impl IRTask {
    pub fn new(id: String, ast_task: &ASTTask, block_name: &str) -> Self {
        Self {
            id,
            name: ast_task.name.clone(),
            params: ast_task.params.clone(),
            duration: ast_task.duration.clone(),
            block: block_name.to_string(),
            tags: ast_task.tags.clone(),
            priority: ast_task.priority.clone(),
            depends_on: ast_task.depends_on.clone(),
            scheduled_start: None,
            scheduled_end: None,
            completed: false,
        }
    }
    
    pub fn duration_minutes(&self) -> u64 {
        self.duration.as_ref().map_or(0, |d| d.minutes)
    }
    
    pub fn with_scheduled_time(mut self, start: DateTime<Local>) -> Self {
        let end = start + Duration::minutes(self.duration_minutes() as i64);
        self.scheduled_start = Some(start);
        self.scheduled_end = Some(end);
        self
    }
    
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
    
    pub fn display_name(&self) -> String {
        if self.params.is_empty() {
            self.name.clone()
        } else {
            format!("{}({})", self.name, self.params.join(", "))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRBlock {
    pub name: String,
    pub tasks: Vec<IRTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRMetadata {
    pub timezone: String,
    pub workday_start: NaiveTime,
    pub workday_end: NaiveTime,
    pub max_parallel: usize,
    pub focus_tags: Vec<String>,
    pub optimization_level: u8,
}

impl Default for IRMetadata {
    fn default() -> Self {
        Self {
            timezone: Local::now().timezone().to_string(),
            workday_start: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            workday_end: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            max_parallel: 1,
            focus_tags: Vec::new(),
            optimization_level: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRProgram {
    pub blocks: Vec<IRBlock>,
    pub metadata: IRMetadata,
    pub task_map: HashMap<String, IRTask>,
}

impl IRProgram {
    pub fn new(blocks: Vec<IRBlock>, metadata: IRMetadata) -> Self {
        let mut task_map = HashMap::new();
        
        // Build a map of task IDs to tasks for quick lookup
        for block in &blocks {
            for task in &block.tasks {
                task_map.insert(task.id.clone(), task.clone());
            }
        }
        
        Self {
            blocks,
            metadata,
            task_map,
        }
    }
    
    pub fn get_task(&self, id: &str) -> Option<&IRTask> {
        self.task_map.get(id)
    }
    
    pub fn all_tasks(&self) -> Vec<&IRTask> {
        self.blocks.iter().flat_map(|b| b.tasks.iter()).collect()
    }
    
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&IRTask> {
        self.all_tasks().into_iter().filter(|t| t.has_tag(tag)).collect()
    }
    
    pub fn total_duration(&self) -> u64 {
        self.all_tasks().iter().map(|t| t.duration_minutes()).sum()
    }
}

pub fn to_ir(ast_blocks: &[ASTBlock], metadata: IRMetadata) -> IRProgram {
    let mut blocks = vec![];
    let mut task_id_counter = 0;
    
    for b in ast_blocks {
        let mut tasks = vec![];
        for t in &b.tasks {
            let id = format!("task_{}", task_id_counter);
            task_id_counter += 1;
            
            tasks.push(IRTask::new(id, t, &b.name));
        }
        blocks.push(IRBlock { name: b.name.clone(), tasks });
    }
    
    IRProgram::new(blocks, metadata)
}
