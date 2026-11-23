use clap::Command;

// gen play command
pub fn gen_play_command() -> Command {
    Command::new("play").about("play audio files")
}
