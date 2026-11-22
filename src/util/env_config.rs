use anyhow::{Context, Result};
use std::env;

pub fn get_music_folder() -> Result<String> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(error) if error.not_found() => {}
        Err(dotenvy::Error::LineParse(..)) => anyhow::bail!("Could not parse .env"),
        Err(error) => return Err(error).context("Could not load .env"),
    }

    Ok(env::var("MUSIC_FOLDER").unwrap_or_else(|_| "/home/pramod/Music".to_owned()))
}
