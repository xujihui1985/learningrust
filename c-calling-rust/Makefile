CC ?= cc
CFLAGS ?= -Wall -Wextra -Werror -std=c11
PROFILE ?= debug
RUST_MANIFEST := rust-lib/Cargo.toml
CARGO_FLAGS :=
ifeq ($(PROFILE),release)
CARGO_FLAGS += --release
endif
RUST_TARGET_DIR := rust-lib/target/$(PROFILE)
RUST_LIB := $(RUST_TARGET_DIR)/librust_lib.a
RUST_HEADER := $(RUST_TARGET_DIR)/include/rust_lib.h
C_BIN := $(RUST_TARGET_DIR)/c-calling-rust

.PHONY: all run clean FORCE

all: $(C_BIN)

rustlib:
	cargo build --manifest-path $(RUST_MANIFEST) $(CARGO_FLAGS)

$(C_BIN): main.c rustlib
	$(CC) $(CFLAGS) -I$(dir $(RUST_HEADER)) main.c $(RUST_LIB) -o $@

run: $(C_BIN)
	./$(C_BIN)

clean:
	cargo clean --manifest-path $(RUST_MANIFEST)
