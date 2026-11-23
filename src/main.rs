mod features;
mod handlers;
mod util;

use clap::{Command, command};
use std::sync::OnceLock;
// ..features
use crate::features::gen_list::gen_list_command;
use crate::features::gen_play::gen_play_command;
// ..handlers
use crate::handlers::help_handler::handle_help;
use crate::handlers::list_handler::handle_list_music;
use crate::handlers::play_handler::handle_play_music;
// ..util
use crate::util::env_config::get_music_folder;

pub const VERSION: &str = "0.2.0";
pub const APP_DESCRIPTION: &str = "WINAMP MP3 CLI - A CLI MP3 Music Player";
pub static MUSIC_FOLDER: OnceLock<String> = OnceLock::new();

fn main() -> anyhow::Result<()> {
    // Music folder loaded once at startup, before command handlers run.
    MUSIC_FOLDER
        .set(get_music_folder()?)
        .map_err(|_| anyhow::anyhow!("Music folder has already been initialized"))?;

    let cli_matches = command!()
        .about(APP_DESCRIPTION)
        .version(VERSION)
        .override_help(handle_help())
        .disable_help_subcommand(true)
        .subcommand(Command::new("help").about("Show this help message"))
        .subcommand(gen_play_command())
        .subcommand(gen_list_command())
        .get_matches();

    // match features by their handlers
    let result = if let Some(matches) = cli_matches.subcommand_matches("play") {
        handle_play_music(matches)
    } else if let Some(matches) = cli_matches.subcommand_matches("ls") {
        handle_list_music(matches)
    } else {
        println!("{}", handle_help());
        Ok(())
    };

    // allow terminal pipe commands
    if result.as_ref().is_err_and(|error| {
        error
            .downcast_ref::<std::io::Error>()
            .is_some_and(|error| error.kind() == std::io::ErrorKind::BrokenPipe)
    }) {
        return Ok(());
    }
    result
}
