use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rsearch")]
#[command(version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Name { keyword: String, path: String },

    Content { keyword: String, path: String },

    Regex { pattern: String, path: String },

    Ext { ext: String, path: String },

    Size { min_size: u64, path: String },

    Recent { days: u64, path: String },

    Duplicate { path: String },

    BuildIndex { path: String },

    SearchIndex { keyword: String },

    Stats { path: String },

    History,
}
