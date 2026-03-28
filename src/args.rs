//! Command line arguments

use clap::Parser;

/// notify-sound - Play a sound on incoming notifications.
#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Wether to enable "Debug mode" (Prints the metadata of incoming notifications)
    #[arg(short, long)]
    pub debug: bool,
}
