use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Block(String),
    Task(String),
    Duration(String),
    Tag(String),
    Priority(String),
    DependsOn(Vec<String>),
    Indent,
    Newline,
    EOF,
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
}

pub type LexerResult<T> = Result<T, LexerError>;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            tokens.push(Token::Newline);
            continue;
        }
        
        // Block definition
        if trimmed.starts_with("@") {
            tokens.push(Token::Block(trimmed[1..].to_string()));
            tokens.push(Token::Newline);
            continue;
        }
        
        // Task with possible attributes
        let mut task_parts = trimmed.split_whitespace().collect::<Vec<_>>();
        let mut task_name = String::new();
        
        // Extract the task name (which might include parameters)
        if !task_parts.is_empty() {
            task_name = task_parts[0].to_string();
            task_parts.remove(0);
        }
        
        tokens.push(Token::Task(task_name));
        
        // Process remaining parts (duration, tags, priority, dependencies)
        for part in task_parts {
            if part.starts_with("[") && part.ends_with("]") {
                // Duration
                tokens.push(Token::Duration(part.trim_matches(['[', ']']).to_string()));
            } else if part.starts_with("#") {
                // Tag
                tokens.push(Token::Tag(part[1..].to_string()));
            } else if part.starts_with("p:") {
                // Priority
                tokens.push(Token::Priority(part[2..].to_string()));
            } else if part.starts_with("after:") {
                // Dependencies
                let deps = part[6..].split(',').map(|s| s.to_string()).collect();
                tokens.push(Token::DependsOn(deps));
            }
        }
        
        tokens.push(Token::Newline);
    }
    
    tokens.push(Token::EOF);
    tokens
}

// Helper function to extract parameters from a task name like "write(report)"
pub fn extract_task_params(task_str: &str) -> (String, Vec<String>) {
    if let Some(open_paren) = task_str.find('(') {
        if let Some(close_paren) = task_str.find(')') {
            if open_paren < close_paren {
                let name = task_str[..open_paren].to_string();
                let params_str = &task_str[open_paren + 1..close_paren];
                let params = params_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                return (name, params);
            }
        }
    }
    
    (task_str.to_string(), Vec::new())
}
