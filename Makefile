PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

install:
	cargo build --release
	cargo install --path .

uninstall:
	cargo uninstall omenrgb
