use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Run the GUI. This is the default command.
    Gui {},
    /// Export Specta bindings for development purposes
    #[cfg(debug_assertions)]
    ExportBindings {},
}

/// Zen and the Automation of Metaprogramming for the Masses
///
/// This is an experimental tool meant for automating programming-related activities,
/// although none have been implemented yet. Blog posts on progress can be found at
/// https://zamm.dev/
#[derive(Parser)]
#[command(name = "zamm")]
#[command(version)]
#[command(about = "Zen and the Automation of Metaprogramming for the Masses")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
