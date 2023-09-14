alias b := build
alias c := clean
alias l := lint
alias r := run
alias t := test

# Compiles the rust source files
build *ARGUMENTS:
	cargo build --release *ARGUMENTS

# Removes temporary files
clean:
	cargo clean

# Lints the rust source files
lint *ARGUMENTS:
	cargo check *ARGUMENTS

# Compiles and executes the main.rs file
run *ARGUMENTS:
	cargo run *ARGUMENTS

# Runs the unittests
test *ARGUMENTS:
	cargo test *ARGUMENTS
