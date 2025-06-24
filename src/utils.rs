use std::path::Path;
use anyhow::{Result, anyhow};

/// Slugify a string for use in filenames
pub fn slugify(input: &str) -> String {
    let mut result = String::new();
    
    // Replace spaces and special characters with dashes
    for c in input.chars() {
        if c.is_alphanumeric() {
            result.push(c.to_ascii_lowercase());
        } else if c.is_whitespace() || c == '-' || c == '_' {
            // Don't add consecutive dashes
            if !result.ends_with('-') {
                result.push('-');
            }
        }
    }
    
    // Trim dashes from the ends
    result = result.trim_matches('-').to_string();
    
    // Ensure the result is not empty
    if result.is_empty() {
        result = "unnamed".to_string();
    }
    
    result
}

/// Format a duration in minutes to a human-readable string
pub fn format_duration(minutes: u64) -> String {
    if minutes < 60 {
        format!("{}m", minutes)
    } else if minutes % 60 == 0 {
        format!("{}h", minutes / 60)
    } else {
        format!("{}h {}m", minutes / 60, minutes % 60)
    }
}

/// Sanitize an output filename
pub fn sanitize_output_name(name: &str, extension: &str) -> String {
    let name = slugify(name);
    
    // Add extension if not present
    if !name.ends_with(&format!(".{}", extension)) {
        format!("{}.{}", name, extension)
    } else {
        name.to_string()
    }
}

/// Round a time to the nearest 15 minutes
pub fn round_to_nearest_15_min(time: chrono::NaiveTime) -> chrono::NaiveTime {
    let minutes = time.minute();
    let rounded_minutes = ((minutes + 7) / 15) * 15;
    
    if rounded_minutes == 60 {
        time.with_minute(0).unwrap().with_hour(time.hour() + 1).unwrap()
    } else {
        time.with_minute(rounded_minutes).unwrap()
    }
}

/// Check if a command is available in the system
pub fn command_exists(command: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("where")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// Ensure a directory exists
pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| anyhow!("Failed to create directory {}: {}", path.display(), e))?;
    }
    Ok(())
}