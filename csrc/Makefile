CC=clang
CFLAGS= "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=wasm32-unknown-unknown" -Wno-unused-parameter "-D__WASM32__=1" "-D__PINK__=1"

all: cutils.o libregexp.o libunicode.o quickjs.o qjs-pink.o

clean:
	rm -rf *.o
