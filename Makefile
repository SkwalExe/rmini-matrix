all: install

install:
	cargo build --release
	sudo cp target/release/rmini-matrix /usr/bin/rmini-matrix

uninstall:
	sudo rm -f /usr/bin/rmini-matrix