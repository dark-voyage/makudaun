use clap::{Parser, Subcommand};

/// Markdown to HTML converter tool
#[derive(Debug, Parser)]
#[command(name = "makudaun")]
#[command(about = "Markdown to HTML converter tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Convert markdown files to html
    #[command(arg_required_else_help = true)]
    Convert {
        /// The remote to clone
        #[clap(short, long)]
        file: String,

        /// Output file
        #[clap(short, long)]
        output: Option<String>,
    },
}