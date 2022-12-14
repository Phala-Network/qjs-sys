.PHONY: all libc clean run

all: libc
	cargo build --release --target wasm32-unknown-unknown

libc: pink-libc/sysroot
	
pink-libc/sysroot:
	make -C pink-libc

clean:
	cargo clean
	rm -rf pink-libc/sysroot
run:
	cargo build --release --target wasm32-wasi
	wasmer run --allow-multiple-wasi-versions target/wasm32-wasi/release/qjs-sys.wasm
