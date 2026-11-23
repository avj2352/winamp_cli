use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use std::io::{self, Write};

use crate::util::audio_files::{display_name, music_files};

pub fn handle_list_music(_args: &ArgMatches) -> Result<()> {
    let files = music_files()?;
    let mut out = io::stdout().lock();
    if files.is_empty() {
        writeln!(
            out,
            "{}",
            "No supported audio files found in MUSIC_FOLDER.".yellow()
        )?;
    } else {
        for path in files {
            writeln!(out, "{}", display_name(&path).cyan())?;
        }
    }
    Ok(())
}
