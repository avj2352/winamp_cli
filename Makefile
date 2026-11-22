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
