.DEAFAULT_GOAL:= all

.PHONY: clippy generate bindings

generate:
	cargo build -p libgbrainy && cargo build -p ffi

bindings: generate
	pushd mobile && \
		flutter pub run ffigen && \
		popd

clippy:
	cargo clippy -- -D warnings

all: clippy bindings
