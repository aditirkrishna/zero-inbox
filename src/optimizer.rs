use crate::ast::Priority;
use crate::ir::{IRProgram, IRTask, IRBlock};
use log::info;

pub fn optimize(program: &mut IRProgram) -> &mut IRProgram {
    info!("Optimizing program with {} blocks", program.blocks.len());
    
    match program.metadata.optimization_level {
        0 => {
            // No optimization
            info!("Optimization level 0: No optimization applied");
        },
        1 => {
            // Basic optimization: sort by priority within blocks
            info!("Optimization level 1: Sorting tasks by priority within blocks");
            sort_by_priority(program);
        },
        2 => {
            // Advanced optimization: sort by priority and resolve dependencies
            info!("Optimization level 2: Sorting tasks by priority and resolving dependencies");
            sort_by_priority(program);
            resolve_dependencies(program);
        },
        _ => {
            // Maximum optimization: sort by priority, resolve dependencies, and group by tags
            info!("Optimization level 3+: Full optimization");
            sort_by_priority(program);
            resolve_dependencies(program);
            group_by_tags(program);
        }
    }
    
    program
}

fn sort_by_priority(program: &mut IRProgram) {
    for block in &mut program.blocks {
        block.tasks.sort_by(|a, b| {
            // Sort by priority (higher priority first)
            let priority_cmp = priority_to_value(&b.priority).cmp(&priority_to_value(&a.priority));
            
            if priority_cmp == std::cmp::Ordering::Equal {
                // If priorities are equal, sort by duration (shorter tasks first)
                a.duration_minutes().cmp(&b.duration_minutes())
            } else {
                priority_cmp
            }
        });
    }
}

fn resolve_dependencies(program: &mut IRProgram) {
    // Create a new set of blocks with reordered tasks
    let mut new_blocks = Vec::new();
    
    for block in &program.blocks {
        let mut new_block = IRBlock {
            name: block.name.clone(),
            tasks: Vec::new(),
        };
        
        // Create a map of task names to their dependencies
        let mut dependency_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        let mut task_map: std::collections::HashMap<String, IRTask> = std::collections::HashMap::new();
        
        for task in &block.tasks {
            dependency_map.insert(task.id.clone(), task.depends_on.clone());
            task_map.insert(task.id.clone(), task.clone());
        }
        
        // Perform a topological sort
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        let mut ordered_tasks = Vec::new();
        
        for task_id in dependency_map.keys() {
            if !visited.contains(task_id) {
                topological_sort(
                    task_id,
                    &dependency_map,
                    &mut visited,
                    &mut temp_visited,
                    &mut ordered_tasks,
                );
            }
        }
        
        // Add tasks in the correct order
        for task_id in ordered_tasks {
            if let Some(task) = task_map.get(&task_id) {
                new_block.tasks.push(task.clone());
            }
        }
        
        new_blocks.push(new_block);
    }
    
    program.blocks = new_blocks;
}

fn topological_sort(
    task_id: &str,
    dependency_map: &std::collections::HashMap<String, Vec<String>>,
    visited: &mut std::collections::HashSet<String>,
    temp_visited: &mut std::collections::HashSet<String>,
    ordered_tasks: &mut Vec<String>,
) {
    // Check for cycles
    if temp_visited.contains(task_id) {
        // Cycle detected, skip this task
        return;
    }
    
    // If already visited, skip
    if visited.contains(task_id) {
        return;
    }
    
    // Mark as temporarily visited
    temp_visited.insert(task_id.to_string());
    
    // Visit all dependencies
    if let Some(dependencies) = dependency_map.get(task_id) {
        for dep in dependencies {
            topological_sort(dep, dependency_map, visited, temp_visited, ordered_tasks);
        }
    }
    
    // Mark as visited
    visited.insert(task_id.to_string());
    temp_visited.remove(task_id);
    
    // Add to ordered list
    ordered_tasks.push(task_id.to_string());
}

fn group_by_tags(program: &mut IRProgram) {
    // Group tasks by tags within each block
    for block in &mut program.blocks {
        // First, collect all unique tags
        let mut all_tags = std::collections::HashSet::new();
        for task in &block.tasks {
            for tag in &task.tags {
                all_tags.insert(tag.clone());
            }
        }
        
        // If there are focus tags specified, only use those
        let tags_to_use = if !program.metadata.focus_tags.is_empty() {
            program.metadata.focus_tags.clone()
        } else {
            all_tags.into_iter().collect()
        };
        
        // Group tasks by tag
        let mut grouped_tasks = Vec::new();
        
        // First add tasks with the specified tags
        for tag in &tags_to_use {
            let mut tasks_with_tag: Vec<_> = block.tasks.iter()
                .filter(|t| t.has_tag(tag))
                .cloned()
                .collect();
            
            // Sort tasks with the same tag by priority
            tasks_with_tag.sort_by(|a, b| {
                priority_to_value(&b.priority).cmp(&priority_to_value(&a.priority))
            });
            
            grouped_tasks.extend(tasks_with_tag);
        }
        
        // Then add remaining tasks
        let tagged_task_ids: std::collections::HashSet<_> = grouped_tasks.iter()
            .map(|t| t.id.clone())
            .collect();
        
        let remaining_tasks: Vec<_> = block.tasks.iter()
            .filter(|t| !tagged_task_ids.contains(&t.id))
            .cloned()
            .collect();
        
        grouped_tasks.extend(remaining_tasks);
        
        // Remove duplicates while preserving order
        let mut seen_ids = std::collections::HashSet::new();
        block.tasks = grouped_tasks.into_iter()
            .filter(|task| seen_ids.insert(task.id.clone()))
            .collect();
    }
}

fn priority_to_value(priority: &Priority) -> u8 {
    match priority {
        Priority::Low => 0,
        Priority::Medium => 1,
        Priority::High => 2,
        Priority::Critical => 3,
    }
}