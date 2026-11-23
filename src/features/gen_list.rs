use clap::Command;

// gen play command
pub fn gen_list_command() -> Command {
    Command::new("ls").about("list audio files")
}
