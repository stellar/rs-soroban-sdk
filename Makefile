all:
	cargo build
	cargo test
	cargo build --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
	for i in *.wasm ; do \
		wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
		ls -l "$$i"; \
	done

tiny:
	cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
	for i in *.wasm ; do \
		wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
		ls -l "$$i"; \
	done

fmt:
	rustfmt $$(find . -type f -name '*.rs' -print)

clean:
	cargo clean
