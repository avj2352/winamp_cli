# Winamp MP3 CLI - A CLI application, to play local drive MP3 music

![Version 1.1.1](https://img.shields.io/badge/version-1.1.1-blue)


✨ WINAMP MP3 CLI - A CLI MP3 Music Player✨

A Rust powered CLI application, that lists and plays your MP3, audio files

## Run locally

With Rust and Cargo installed, run this from the repository root:

```sh
cargo run
```

## List and play music

```sh
MUSIC_FOLDER=/path/to/music cargo run -- ls
MUSIC_FOLDER=/path/to/music cargo run -- play
```

Supports MP3, WAV, FLAC, OGG and OGA files directly inside the music folder.
Use arrow keys and Enter to choose a track. Playback shows a live waveform;
Left/Right seeks backward/forward 10 seconds, Space pauses/resumes, and q, Esc or Ctrl-C stops.
See [seeking controls](docs/seeking-controls.md) for details.
Use `play --loop` to repeat the selected track with a `Loop: ON` indicator.
See [loop playback](docs/loop-playback.md) for details.

Linux builds require `pkg-config` and ALSA development headers:
`sudo apt-get install -y pkg-config libasound2-dev`.
See [audio listing and playback](docs/audio-listing-and-playback.md) for details.

## Makefile

The following are Makefile actions -

```bash
tree:
	@echo "🌳 Show project tree.."
	tree -L 5 -I target -I release .

clean:
	@echo "Cleanup dependencies.."
	rm -rf target
	rm -rf release
	rm -f Cargo.lock

build:
	@echo "Initializing fresh build.."
	rm -rf release
	cargo build --release
	@echo "Created CLI release build.."
	@echo "Transferring.."
	mkdir release
	@echo "Packaging executable and release information.."
	cp ./target/release/winamp_cli release/winamp
	cp ./*.md release/
	@echo "Packaging / Release completed !!"

```


