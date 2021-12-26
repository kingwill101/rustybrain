.DEAFAULT_GOAL:= all

.PHONY: clippy generate bindings publish

install:
	cargo install wasm-pack
	cargo install cargo-generate
	npm install -g dart_js_facade_gen

generate:
	cargo build -p libgbrainy && cargo build -p ffi
	pushd wasm && wasm-pack build --release --target web
	rm -rf wasm/lib/src/flutter_web_wasm_base.dart
	dart_js_facade_gen wasm/pkg/librustybrain_wasm.d.ts | tee wasm/lib/src/flutter_web_wasm_base.dart

bindings: generate
	pushd mobile && \
		flutter pub run ffigen && \
		popd

clippy:
	cargo clippy -- -D warnings

publish:
	wasm-pack publish

all: clippy bindings
