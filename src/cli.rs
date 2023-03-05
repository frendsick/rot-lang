use clap::{Args, Parser, Subcommand};

/// Rot compiler
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct RotArgs {
    #[command(subcommand)]
    pub action: CliAction,
}

#[derive(Debug, Subcommand)]
pub enum CliAction {
    /// Compile a Rot program
    Compile(CompilationTarget),
}

#[derive(Debug, Args)]
pub struct CompilationTarget {
    /// Rot code file
    pub rot_file: String,
    /// Output file
    #[arg(short, long, value_name="FILE")]
    pub out: Option<String>,
    /// Save the generated assembly file
    #[arg(short, long)]
    pub save_asm: bool,
    /// Output compilation steps
    #[arg(short, long)]
    pub verbose: bool,
}
