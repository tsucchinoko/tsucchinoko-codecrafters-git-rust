use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Initialise a new repository
    Init,

    /// Provide content or type and size information for repository objects
    CatFile {
        /// Pretty print the object
        #[arg(short)]
        pretty_print: bool,

        /// The object to cat
        hash: String,
    },

    /// Compute object ID and optionally creates a blob from a file
    HashObject {
        /// Write the object into the object database
        #[arg(short)]
        write: bool,

        /// The file to hash
        file: PathBuf,
    },
}