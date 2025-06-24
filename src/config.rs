use crate::ir::IRMetadata;
use crate::scheduler::ScheduleMode;
use crate::codegen::OutputFormat;
use chrono::NaiveTime;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub output_format: String,
    pub output_file: Option<String>,
    pub workday_start: String,
    pub workday_end: String,
    pub schedule_mode: String,
    pub optimization_level: u8,
    pub focus_tags: Vec<String>,
    pub max_parallel: usize,
    pub deepwork_tag: String,
    pub dry_run: bool,
    pub show_ir: bool,
    pub visualize_schedule: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_format: "markdown".to_string(),
            output_file: None,
            workday_start: "09:00".to_string(),
            workday_end: "17:00".to_string(),
            schedule_mode: "naive".to_string(),
            optimization_level: 1,
            focus_tags: Vec::new(),
            max_parallel: 1,
            deepwork_tag: "deepwork".to_string(),
            dry_run: false,
            show_ir: false,
            visualize_schedule: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Look for config file in standard locations
        let config_paths = [
            dirs::config_dir().map(|p| p.join("zibox").join(".ziboxrc")),
            dirs::home_dir().map(|p| p.join(".ziboxrc")),
            Some(PathBuf::from(".ziboxrc")),
        ];
        
        for path in config_paths.iter().flatten() {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                return toml::from_str(&content).map_err(|e| anyhow!("Failed to parse config: {}", e));
            }
        }
        
        // No config file found, return default
        Ok(Config::default())
    }
    
    pub fn to_ir_metadata(&self) -> Result<IRMetadata> {
        // Parse workday times
        let workday_start = parse_time(&self.workday_start)?;
        let workday_end = parse_time(&self.workday_end)?;
        
        Ok(IRMetadata {
            timezone: chrono::Local::now().timezone().to_string(),
            workday_start,
            workday_end,
            max_parallel: self.max_parallel,
            focus_tags: self.focus_tags.clone(),
            optimization_level: self.optimization_level,
        })
    }
    
    pub fn get_output_format(&self) -> Result<OutputFormat> {
        OutputFormat::from_str(&self.output_format)
            .ok_or_else(|| anyhow!("Invalid output format: {}", self.output_format))
    }
    
    pub fn get_schedule_mode(&self) -> Result<ScheduleMode> {
        ScheduleMode::from_str(&self.schedule_mode)
            .ok_or_else(|| anyhow!("Invalid schedule mode: {}", self.schedule_mode))
    }
}

fn parse_time(time_str: &str) -> Result<NaiveTime> {
    // Try to parse HH:MM format
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
        return Ok(time);
    }
    
    // Try to parse H:MM format
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%-H:%M") {
        return Ok(time);
    }
    
    Err(anyhow!("Invalid time format: {}", time_str))
}