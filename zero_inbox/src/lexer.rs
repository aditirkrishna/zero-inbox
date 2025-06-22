use logos::{Logos, SpannedIter};
use std::fmt;

/// Represents different types of tokens in the Zero Inbox DSL
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\r]+")] // Skip whitespace
pub enum Token {
    // Block header: @morning, @evening, etc.
    #[token("@")]
    At,
    
    // Task definition: task_name(param1, param2)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*\([^)]*\)")]
    Task(String),
    
    // Duration: [2h], [30m], etc.
    #[regex(r"\[[0-9]+[hHmM]\]")]
    Duration(String),
    
    // Newline (used for block separation)
    #[regex("\n")]
    Newline,
    
    // Indentation (spaces at start of line)
    #[regex(r"^[ \t]+", priority = 100)]
    Indent,
    
    // Error token for invalid input
    #[error]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::At => write!(f, "@"),
            Token::Task(s) => write!(f, "{}", s),
            Token::Duration(s) => write!(f, "{}", s),
            Token::Newline => write!(f, "newline"),
            Token::Indent => write!(f, "indent"),
            Token::Error => write!(f, "error"),
        }
    }
}

/// A wrapper around the Logos lexer that provides a more convenient interface
pub struct Lexer<'a> {
    inner: SpannedIter<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Lexer {
            inner: Token::lexer(input).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, std::ops::Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_block_header() {
        let input = "@morning";
        let mut lexer = Token::lexer(input);
        
        assert_eq!(lexer.next(), Some(Token::At));
        assert_eq!(lexer.slice(), "@");
        
        assert_eq!(lexer.next(), Some(Token::Task("morning".to_string())));
        assert_eq!(lexer.slice(), "morning");
        
        assert_eq!(lexer.next(), None);
    }
    
    #[test]
    fn test_tokenize_task_with_duration() {
        let input = "  write(report) [2h]";
        let mut lexer = Token::lexer(input);
        
        assert_eq!(lexer.next(), Some(Token::Indent));
        assert_eq!(lexer.slice(), "  ");
        
        assert_eq!(lexer.next(), Some(Token::Task("write(report)".to_string())));
        assert_eq!(lexer.slice(), "write(report)");
        
        assert_eq!(lexer.next(), Some(Token::Duration("[2h]".to_string())));
        assert_eq!(lexer.slice(), "[2h]");
        
        assert_eq!(lexer.next(), None);
    }
    
    #[test]
    fn test_tokenize_multiline() {
        let input = "@morning\n  task1()\n  task2() [30m]\n";
        let tokens: Vec<_> = Token::lexer(input).collect();
        
        assert_eq!(tokens, vec![
            Token::At,
            Token::Task("morning".to_string()),
            Token::Newline,
            Token::Indent,
            Token::Task("task1()".to_string()),
            Token::Newline,
            Token::Indent,
            Token::Task("task2()".to_string()),
            Token::Duration("[30m]".to_string()),
            Token::Newline,
        ]);
    }
}
