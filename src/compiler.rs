use crate::ast::generate_ast;
use crate::class::program::Program;
use crate::class::token::Token;
use crate::lexer::tokenize_code_file;

#[derive(Debug)]
pub enum CompilerError {
    IOError(std::io::Error),
    ParserError(String),
}

pub fn compile_rot_file(rot_file: &str, _out_file: Option<String>) -> Result<(), CompilerError> {
    let tokens: Vec<Token> = tokenize_code_file(&rot_file)?;
    let program: Program = generate_ast(&tokens)?;
    dbg!(&program);
    // let functions: Vec<Function> = parse_functions(tokens)?;
    // TODO: Generate abstract syntax tree (AST)
    // TODO: Generate assembly code
    // TODO: Compile the program
    Ok(())
}
