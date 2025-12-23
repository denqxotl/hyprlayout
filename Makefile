BINARY=hyprlayout
install:
	@cargo build --release
	@sudo cp target/release/${BINARY} /usr/local/bin/${BINARY}
	@echo "Installed $(BINARY) to /usr/local/bin"

uninstall:
	@sudo rm /usr/local/bin/${BINARY}
