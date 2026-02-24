PREFIX ?= /usr/local
BINDIR := $(PREFIX)/bin
NAME   := omenrgb
TARGET := target/release/$(NAME)
CARGO  ?= cargo

.PHONY: build install install-local uninstall uninstall-local clean

#  Build binary release
build:
	$(CARGO) build --release

# Local install(cargo-managed)
install-local: build
	$(CARGO) install --path .

# System-wide install
install:
	install -Dm755 $(TARGET) $(BINDIR)/$(NAME)

# Remove system-wide install
uninstall:
	rm -f $(BINDIR)/$(NAME)

# Local uninstall(cargo-managed)
uninstall-local:
	$(CARGO) uninstall $(NAME)

# Clean build artifacts
clean:
	$(CARGO) clean
