use crate::{class::function::Function, constant::MAIN_FUNCTION_NAME};
use std::format as f;

pub fn generate_assembly(functions: Vec<Function>) -> String {
    let mut assembly: String = String::new();
    for function in functions {
        assembly.push_str(generate_function_assembly(&function).as_str());
    }
    assembly
}

fn generate_function_assembly(function: &Function) -> String {
    let mut assembly: String = function_start_assembly(&function.name);
    for token in &function.tokens {
    }
    assembly.push_str(function_end_assembly());
    assembly
}

fn function_start_assembly(function_name: &str) -> String {
    let mut assembly = String::new();
    assembly.push_str(&f!("{function_name}:\n"));
    assembly.push_str("  push rbp\n");
    assembly.push_str("  mov rbp, rsp\n");
    assembly
}

fn function_end_assembly<'a>() -> &'a str {
    "  pop rbp\n  ret\n\n"
}
