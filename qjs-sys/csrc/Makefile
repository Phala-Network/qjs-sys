CFLAGS="-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-I" "../pink-libc/sysroot/include" "-funsigned-char" "-D_GNU_SOURCE" "-D__pink__=1" "-I."
CC=clang

SRC= \
    quickjs/cutils.c \
	quickjs/libregexp.c \
	quickjs/libunicode.c \
	quickjs/quickjs.c \
	quickjs/libbf.c \

OBJ = $(SRC:.c=.o)

.PHONY: bindings clean obj

obj: $(OBJ)

bindings:
	./c2rust.sh

clean:
	rm -rf quickjs/*.o *.o ext/*.o inline32.c inline64.c *.rs compile_commands.json

# Compile the C source files into object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
