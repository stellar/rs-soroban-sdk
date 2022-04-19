all:
	cargo build
	cargo test
	cargo build --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
	for i in *.wasm ; do \
		wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
		ls -l "$$i"; \
	done

clean:
	cargo clean

