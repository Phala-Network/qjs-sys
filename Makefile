.PHONY: all libc

all: libc
	cargo build --release --target wasm32-unknown-unknown

libc: pink-libc/sysroot
	
pink-libc/sysroot:
	make -C pink-libc

clean:
	cargo clean
	rm -rf pink-libc/sysroot
