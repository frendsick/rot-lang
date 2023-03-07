use clap::Parser;

use cli::{CliAction, RotArgs};
use compiler::compile_rot_file;

mod class;
mod cli;
mod compiler;
mod constant;
mod data_types;
mod lexer;

fn main() {
    cli_action(RotArgs::parse());
}

fn cli_action(args: RotArgs) {
    match args.action {
        // ./rot-rust compile <ROT_FILE>
        // TODO: CompilerError handling
        CliAction::Compile(target) => compile_rot_file(&target.rot_file, target.out).unwrap(),
    }
}
