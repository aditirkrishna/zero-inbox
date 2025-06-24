use crate::ast::{Block, Task, Priority, TaskDuration};
use crate::lexer::{Token, extract_task_params};
use std::str::FromStr;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    
    #[error("Invalid priority: {0}")]
    InvalidPriority(String),
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Block>, ParseError> {
    let mut i = 0;
    let mut blocks = vec![];
    let mut current_block_name = String::from("default");
    
    // Create a default block to hold tasks that aren't in a specific block
    let mut default_block = Block::new(current_block_name.clone());
    
    while i < tokens.len() {
        match &tokens[i] {
            Token::Block(name) => {
                // If we've collected tasks in the current block, add it to blocks
                if !default_block.tasks.is_empty() {
                    blocks.push(default_block);
                    default_block = Block::new(name.clone());
                }
                
                current_block_name = name.clone();
                let mut block = Block::new(current_block_name.clone());
                
                i += 1;
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Task(task_str) => {
                            let (name, params) = extract_task_params(task_str);
                            let mut task = Task::new(name).with_params(params);
                            
                            i += 1;
                            
                            // Process task attributes (duration, tags, priority, dependencies)
                            let mut tags = HashSet::new();
                            let mut priority = Priority::default();
                            let mut depends_on = Vec::new();
                            let mut duration = None;
                            
                            while i < tokens.len() {
                                match &tokens[i] {
                                    Token::Duration(dur_str) => {
                                        duration = Some(FromStr::from_str(dur_str)
                                            .map_err(|_| ParseError::InvalidDuration(dur_str.clone()))?);
                                        i += 1;
                                    },
                                    Token::Tag(tag) => {
                                        tags.insert(tag.clone());
                                        i += 1;
                                    },
                                    Token::Priority(prio_str) => {
                                        priority = FromStr::from_str(prio_str)
                                            .map_err(|_| ParseError::InvalidPriority(prio_str.clone()))?;
                                        i += 1;
                                    },
                                    Token::DependsOn(deps) => {
                                        depends_on = deps.clone();
                                        i += 1;
                                    },
                                    Token::Newline | Token::Task(_) | Token::Block(_) | Token::EOF => break,
                                    other => return Err(ParseError::UnexpectedToken(format!("{:?}", other))),
                                }
                            }
                            
                            task = task
                                .with_duration(duration)
                                .with_tags(tags)
                                .with_priority(priority)
                                .with_dependencies(depends_on);
                                
                            block.add_task(task);
                        },
                        Token::Newline => i += 1,
                        Token::Block(_) | Token::EOF => break,
                        other => return Err(ParseError::UnexpectedToken(format!("{:?}", other))),
                    }
                }
                
                blocks.push(block);
            },
            Token::Task(task_str) => {
                // Task outside of a block goes to the default block
                let (name, params) = extract_task_params(task_str);
                let mut task = Task::new(name).with_params(params);
                
                i += 1;
                
                // Process task attributes
                let mut tags = HashSet::new();
                let mut priority = Priority::default();
                let mut depends_on = Vec::new();
                let mut duration = None;
                
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Duration(dur_str) => {
                            duration = Some(FromStr::from_str(dur_str)
                                .map_err(|_| ParseError::InvalidDuration(dur_str.clone()))?);
                            i += 1;
                        },
                        Token::Tag(tag) => {
                            tags.insert(tag.clone());
                            i += 1;
                        },
                        Token::Priority(prio_str) => {
                            priority = FromStr::from_str(prio_str)
                                .map_err(|_| ParseError::InvalidPriority(prio_str.clone()))?;
                            i += 1;
                        },
                        Token::DependsOn(deps) => {
                            depends_on = deps.clone();
                            i += 1;
                        },
                        Token::Newline | Token::Task(_) | Token::Block(_) | Token::EOF => break,
                        other => return Err(ParseError::UnexpectedToken(format!("{:?}", other))),
                    }
                }
                
                task = task
                    .with_duration(duration)
                    .with_tags(tags)
                    .with_priority(priority)
                    .with_dependencies(depends_on);
                    
                default_block.add_task(task);
            },
            Token::Newline => i += 1,
            Token::EOF => break,
            other => return Err(ParseError::UnexpectedToken(format!("{:?}", other))),
        }
    }
    
    // Add the default block if it has tasks
    if !default_block.tasks.is_empty() {
        blocks.push(default_block);
    }
    
    Ok(blocks)
}
