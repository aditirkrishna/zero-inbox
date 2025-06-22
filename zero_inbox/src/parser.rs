use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{Block, Program, Task, TaskDuration};
use crate::error::{Error, Result};
use crate::lexer::{Lexer, Token};

/// Parser for the Zero Inbox DSL
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    current_token: Option<(Token, std::ops::Range<usize>)>,
}

impl<'a> Parser<'a> {
    /// Create a new parser for the given input
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let mut parser = Self {
            lexer: lexer.peekable(),
            current_token: None,
        };
        // Initialize first token
        parser.advance();
        parser
    }

    /// Parse the entire input into a Program
    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();
        
        while self.current_token.is_some() {
            if let Some(block) = self.parse_block()? {
                program.add_block(block);
            } else {
                // Skip any unrecognized tokens
                self.advance();
            }
        }
        
        Ok(program)
    }
    
    /// Parse a block (starts with @)
    fn parse_block(&mut self) -> Result<Option<Block>> {
        // Check for @ symbol
        if !matches!(self.current_token, Some((Token::At, _))) {
            return Ok(None);
        }
        
        self.advance(); // Consume @
        
        // Get block name
        let block_name = match self.current_token.take() {
            Some((Token::Task(name), _)) => name,
            _ => return Err(Error::ParseError {
                line: 1, // TODO: Track line numbers
                col: 1,  // TODO: Track column numbers
                message: "Expected block name after @".to_string(),
            }),
        };
        
        self.advance(); // Consume block name
        
        let mut block = Block::new(&block_name);
        
        // Parse tasks until next block or EOF
        while let Some(task) = self.parse_task()? {
            block.add_task(task);
        }
        
        Ok(Some(block))
    }
    
    /// Parse a task line (indented with optional duration)
    fn parse_task(&mut self) -> Result<Option<Task>> {
        // Skip newlines and indentation
        while matches!(self.current_token, Some((Token::Newline, _)) | Some((Token::Indent, _))) {
            self.advance();
        }
        
        // Check for task token
        let task_str = match self.current_token.take() {
            Some((Token::Task(s), _)) => s,
            _ => return Ok(None), // Not a task line
        };
        
        // Parse the task
        let mut task = Task::from_str(&task_str).ok_or_else(|| Error::ParseError {
            line: 1, // TODO: Track line numbers
            col: 1,  // TODO: Track column numbers
            message: format!("Invalid task format: {}", task_str),
        })?;
        
        self.advance(); // Consume task token
        
        // Check for duration
        if let Some((Token::Duration(dur_str), _)) = &self.current_token {
            if let Some(duration) = TaskDuration::from_str(dur_str) {
                task.duration = Some(duration);
                self.advance(); // Consume duration
            }
        }
        
        // Skip to end of line
        while !matches!(self.current_token, Some((Token::Newline, _)) | None) {
            self.advance();
        }
        
        Ok(Some(task))
    }
    
    /// Advance to the next token
    fn advance(&mut self) {
        self.current_token = self.lexer.next();
    }
}

/// Parse a Zero Inbox program from a string
pub fn parse(input: &str) -> Result<Program> {
    let mut parser = Parser::new(input);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::TaskDuration;
    
    #[test]
    fn test_parse_simple_program() {
        let input = r#"
@morning
  check_email()
  write(report) [2h]
  
@evening
  review(PRs) [1h]
"#;
        
        let program = parse(input).unwrap();
        assert_eq!(program.blocks.len(), 2);
        
        let morning = &program.blocks[0];
        assert_eq!(morning.name, "morning");
        assert_eq!(morning.tasks.len(), 2);
        assert_eq!(morning.tasks[0].name, "check_email");
        assert_eq!(morning.tasks[1].name, "write");
        assert_eq!(morning.tasks[1].duration.as_ref().unwrap().minutes, 120);
        
        let evening = &program.blocks[1];
        assert_eq!(evening.name, "evening");
        assert_eq!(evening.tasks[0].name, "review");
        assert_eq!(evening.tasks[0].duration.as_ref().unwrap().minutes, 60);
    }
    
    #[test]
    fn test_parse_invalid_syntax() {
        // Missing block name after @
        let input = "@\n  task()";
        assert!(parse(input).is_err());
        
        // Invalid task format
        let input = "@block\n  invalid-task";
        assert!(parse(input).is_ok()); // Parser is permissive for now
    }
}
