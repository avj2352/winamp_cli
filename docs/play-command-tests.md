# Play command unit tests (1.0.2)

Unit tests in `src/features/gen_play.rs` check that omitting `--loop` disables
looping, a bare `--loop` enables it, and explicit `true`/`false` values work
with both space and equals syntax. Invalid values (`yes`, `1`, an empty
string and uppercase `TRUE`) must return Clap's `InvalidValue` error.

Run `cargo test features::gen_play::tests` to execute these tests without
requiring music files, an interactive terminal or an audio output device.
The command file is formatted with rustfmt. Package and application versions
and the README badge are bumped to 1.0.2, with a matching changelog entry.
