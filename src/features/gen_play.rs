use clap::{Arg, ArgAction, Command, value_parser};

// gen play command
pub fn gen_play_command() -> Command {
    Command::new("play").about("play audio files").arg(
        Arg::new("loop")
            .long("loop")
            .help("Repeat the selected track until stopped (true/false)")
            .value_name("BOOL")
            .value_parser(value_parser!(bool))
            .num_args(0..=1)
            .default_value("false")
            .default_missing_value("true")
            .action(ArgAction::Set),
    )
}

#[cfg(test)]
mod tests {
    use super::gen_play_command;
    use clap::error::ErrorKind;

    #[test]
    fn loop_defaults_to_false() {
        let matches = gen_play_command().try_get_matches_from(["play"]).unwrap();
        assert_eq!(matches.get_one::<bool>("loop"), Some(&false));
    }

    #[test]
    fn bare_loop_flag_enables_looping() {
        let matches = gen_play_command()
            .try_get_matches_from(["play", "--loop"])
            .unwrap();
        assert_eq!(matches.get_one::<bool>("loop"), Some(&true));
    }

    #[test]
    fn loop_accepts_explicit_boolean_values() {
        for (args, expected) in [
            (vec!["play", "--loop", "true"], true),
            (vec!["play", "--loop", "false"], false),
            (vec!["play", "--loop=true"], true),
            (vec!["play", "--loop=false"], false),
        ] {
            let matches = gen_play_command().try_get_matches_from(args).unwrap();
            assert_eq!(matches.get_one::<bool>("loop"), Some(&expected));
        }
    }

    #[test]
    fn loop_rejects_invalid_boolean_values() {
        for value in ["yes", "1", "", "TRUE"] {
            let error = gen_play_command()
                .try_get_matches_from(["play", "--loop", value])
                .unwrap_err();
            assert_eq!(error.kind(), ErrorKind::InvalidValue);
        }
    }
}
