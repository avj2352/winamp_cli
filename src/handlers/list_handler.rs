use anyhow::{Context, Result};
use clap::ArgMatches;
use colored::Colorize;
use std::io::{self, Write};

use crate::util::audio_files::{display_name, music_files};

pub fn handle_list_music(_args: &ArgMatches) -> Result<()> {
    let files = music_files()?;
    let folder = crate::MUSIC_FOLDER
        .get()
        .context("Music folder is not initialized")?;
    let mut out = io::stdout().lock();
    writeln!(out, "MUSIC_FOLDER: {}", folder.yellow().bold())?;
    if files.is_empty() {
        writeln!(
            out,
            "{}",
            "No supported audio files found in MUSIC_FOLDER.".yellow()
        )?;
    } else {
        writeln!(out)?;
        for path in files {
            writeln!(out, "  {} {}", "•".cyan(), display_name(&path).cyan())?;
        }
        writeln!(out)?;
    }
    Ok(())
}
