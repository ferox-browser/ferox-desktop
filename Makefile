APPLICATION_NAME := ferox
APPLICATION_ID := org.ferox.ferox
APPLICATION_VERSION := 0.1.0

# Build debug
target/debug/${APPLICATION_NAME}:
	cargo build

target/release/${APPLICATION_NAME}:
	cargo build --release