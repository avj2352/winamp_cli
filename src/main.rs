mod handlers;
mod util;

use clap::{Command, command};
// ..handlers
use crate::handlers::help_handler::handle_help;
// ..util
use crate::util::env_config::get_music_folder;

pub const VERSION: &str = "0.1.4";
pub const APP_DESCRIPTION: &str = "WINAMP MP3 CLI - A CLI MP3 Music Player";

fn main() -> anyhow::Result<()> {
    let _music_folder = get_music_folder()?;

    let mut cli = command!()
        .about(APP_DESCRIPTION)
        .version(VERSION)
        .override_help(handle_help())
        .disable_help_subcommand(true)
        .subcommand(Command::new("help").about("Show this help message"));

    let matches = cli.get_matches_mut();
    if matches.subcommand_name().is_none() || matches.subcommand_name() == Some("help") {
        cli.print_help()?;
    }
    Ok(())
}
