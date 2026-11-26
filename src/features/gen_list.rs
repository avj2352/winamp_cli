use clap::Command;

// Generate the list command.
pub fn gen_list_command() -> Command {
    Command::new("ls").about("list audio files and show the MUSIC_FOLDER path")
}
