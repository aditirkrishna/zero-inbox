use std::fmt;
use std::path::PathBuf;

/// The main error type for the Zero Inbox compiler
#[derive(Debug)]
pub enum Error {
    /// I/O error when reading/writing files
    IoError(std::io::Error),
    
    /// Error during lexing (tokenization)
    LexError {
        line: usize,
        col: usize,
        message: String,
    },
    
    /// Error during parsing
    ParseError {
        line: usize,
        col: usize,
        message: String,
    },
    
    /// Semantic error (e.g., undefined block reference)
    SemanticError(String),
    
    /// File not found
    FileNotFound(PathBuf),
    
    /// Other errors
    Other(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "I/O error: {}", e),
            Error::LexError { line, col, message } => 
                write!(f, "Lexer error at line {}:{} - {}", line, col, message),
            Error::ParseError { line, col, message } => 
                write!(f, "Parser error at line {}:{} - {}", line, col, message),
            Error::SemanticError(msg) => write!(f, "Semantic error: {}", msg),
            Error::FileNotFound(path) => 
                write!(f, "File not found: {}", path.display()),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Other(msg.to_string())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Other(msg)
    }
}

/// A type alias for `Result<T, Error>`
pub type Result<T> = std::result::Result<T, Error>;
