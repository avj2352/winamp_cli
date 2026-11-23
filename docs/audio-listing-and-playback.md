# Audio listing and playback (0.2.0)

`winamp ls` scans `MUSIC_FOLDER` and prints supported audio filenames, sorted by filename, one per line in cyan. Since 0.2.1, each filename has an indented bullet, with a blank line before and after the list. The scan is nonrecursive and accepts MP3, WAV, FLAC, OGG and OGA extensions regardless of case. Directories and unrelated files are excluded. Extensions identify candidates; corrupt files or unsupported codecs produce a decode error when selected. Symlinks to regular audio files are accepted. Missing or unreadable folders return contextual errors; empty folders show a friendly message. Terminal control characters in names are escaped for safe display; original paths are retained for playback.

`winamp play` uses the same scan and opens a scrolling filename selector. Use arrow keys to select and Enter to play; Esc or q cancels selection. During playback:

- Space pauses or resumes.
- q, Esc or Ctrl-C stops playback.
- An animated cyan waveform shows recent audio peaks alongside elapsed and total time. Unknown total durations display `--:--`.

The waveform is derived from decoded audio samples, using one peak per 20 ms and retaining at most 80 peaks. It is a rolling amplitude envelope, not a frequency spectrum. Audio-device buffering can cause a small visual lead. Playback streams audio instead of decoding the entire track into memory. The display adjusts to terminal width, and the cursor and raw mode are restored on normal completion, stop or returned errors.

Playback needs an interactive terminal and an available audio output device. Listing supports redirected output and disables colors automatically when appropriate through `colored`. Playback messages and selection use `colored`, `dialoguer` supplies the menu, `crossterm` supplies terminal controls, and `rodio` decodes and plays audio. The shared scan and filename formatting live in `src/util/audio_files.rs`; the existing list handler is named `list_handler.rs` (singular).

## Build and run

On Debian/Ubuntu, install the Linux build prerequisites:

```sh
sudo apt-get update
sudo apt-get install -y pkg-config libasound2-dev
cargo build
MUSIC_FOLDER=/path/to/music cargo run -- ls
MUSIC_FOLDER=/path/to/music cargo run -- play
```

For packaged builds, replace `cargo run --` with `winamp`. Set `MUSIC_FOLDER` through the environment or `.env` as described in the existing configuration documentation.

## Validation

`cargo test` covers filtering, sorting, case-insensitive extensions, nonrecursive scanning, empty/missing folders, safe filename rendering, waveform sample preservation, actual peak extraction and bounded history. Use `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` for static checks. Manual playback validation should use a real audio device to confirm audible output, selection, pause/resume, stop and terminal restoration.

The minor version increases to 0.2.0 for the new commands' functionality, with matching package version, application constant, README badge and changelog.

API references: [Rodio 0.21.1](https://docs.rs/rodio/0.21.1/rodio/) and [Dialoguer Select](https://docs.rs/dialoguer/0.12.0/dialoguer/struct.Select.html).

Validation completed for this change: all five unit tests, build, formatting and strict Clippy checks pass. CLI smoke checks verified sorted filename-only output, a missing-folder error and rejection of piped playback. A pseudo-terminal check using a generated WAV and ALSA null output verified selection, decoding, pause, resume, stop and terminal settings restoration. Audible output on physical speakers was not tested.
