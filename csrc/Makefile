CC=clang
CFLAGS= "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=wasm32-unknown-unknown" -Wno-unused-parameter "-D__pink__=1" "--sysroot=../wasi-libc/sysroot"

all: cutils.o libregexp.o libunicode.o quickjs.o qjs-pink.o

clean:
	rm -rf *.o
