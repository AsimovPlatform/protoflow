CARGO = cargo

all: Cargo.toml
	$(CARGO) build

check: Cargo.toml
	$(CARGO) test

clean: Cargo.toml
	@rm -rf *~ target
	$(CARGO) clean

.PHONY: all check clean
.SECONDARY:
.SUFFIXES:
