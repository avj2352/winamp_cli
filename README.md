# Winamp MP3 CLI - A CLI application, to play local drive MP3 music

![Version 0.1.4](https://img.shields.io/badge/version-0.1.4-blue)


✨ WINAMP MP3 CLI - A CLI MP3 Music Player✨

A Rust powered CLI application, that lists and plays your MP3, audio files

## Run locally

With Rust and Cargo installed, run this from the repository root:

```sh
cargo run
```

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


