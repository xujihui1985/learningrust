GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

clean:
	$(CARGO_BIN) clean
	@rm -f ./c_src/main

gen:
	cbindgen --config cbindgen.toml --crate example01 -o ./include/my_project.h

build:
	$(CARGO_BIN) build
	$(GCC_BIN) -o ./c_src/main ./c_src/main.c -Iinclude -L ./target/debug -lc_call_rust