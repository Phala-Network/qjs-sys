CFLAGS="-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-I" "../pink-libc/sysroot/include" "-funsigned-char" "-DCONFIG_BIGNUM" "-D_GNU_SOURCE" "-D__pink__=1" "-I."
CC=clang

SRC= \
	quickjs-opaque.c \
    cutils.c \
	libregexp.c \
	libunicode.c \
	quickjs.c \
	qjs-pink.c \
	libbf.c \

.PHONY: bindings clean obj

obj: $(SRC:.c=.o)

bindings:
	./c2rust.sh

clean:
	rm -rf *.o ext/*.o inline32.c inline64.c *.rs compile_commands.json

# Compile the C source files into object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
