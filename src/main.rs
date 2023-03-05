use clap::Parser;

use cli::{CliAction, TorthArgs};
use compiler::compile_rot_file;

mod assembly;
mod class;
mod cli;
mod compiler;
mod constant;
mod data_types;
mod intrinsics;
mod lexer;

fn main() {
    cli_action(TorthArgs::parse());
}

fn cli_action(args: TorthArgs) {
    match args.action {
        // ./rot-rust compile <TORTH_FILE>
        // TODO: CompilerError handling
        CliAction::Compile(target) => compile_rot_file(target.rot_file, target.out).unwrap(),
    }
}
