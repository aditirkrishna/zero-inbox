pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ir;
pub mod optimizer;
pub mod scheduler;
pub mod codegen;
pub mod config;
pub mod runtime;
pub mod utils;

use anyhow::{Result, anyhow};
use std::path::Path;
use std::fs;

/// Parse a .zbx file and return the AST
pub fn parse_file(path: &Path) -> Result<Vec<ast::Block>> {
    let input = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read file {}: {}", path.display(), e))?;
    
    let tokens = lexer::tokenize(&input);
    let ast = parser::parse(&tokens)
        .map_err(|e| anyhow!("Failed to parse file {}: {:?}", path.display(), e))?;
    
    Ok(ast)
}

/// Compile a .zbx file to the specified output format
pub fn compile(
    input_path: &Path,
    config: &config::Config,
) -> Result<String> {
    // Parse the input file
    let ast = parse_file(input_path)?;
    
    // Convert to IR
    let metadata = config.to_ir_metadata()?;
    let mut ir_program = ir::to_ir(&ast, metadata);
    
    // Optimize
    optimizer::optimize(&mut ir_program);
    
    // Schedule
    let schedule_mode = config.get_schedule_mode()?;
    let scheduler = scheduler::create_scheduler(
        schedule_mode,
        if config.deepwork_tag.is_empty() { None } else { Some(config.deepwork_tag.clone()) }
    );
    scheduler.schedule(&mut ir_program);
    
    // Generate output
    let output_format = config.get_output_format()?;
    let output = codegen::generate_output(&ir_program, output_format)?;
    
    Ok(output)
}
