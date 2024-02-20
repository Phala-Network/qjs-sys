#!/bin/bash

CC=clang
CFLAGS="-O3 -ffunction-sections -fdata-sections -fPIC -funsigned-char -DCONFIG_BIGNUM -D_GNU_SOURCE -D__pink__=1 -w -I. -I./quickjs"
EXTRA_CFLAGS_32="--target=wasm32-unknown-unknown -I../pink-libc/sysroot/include"
EXTRA_CFLAGS_64=

for arch in 32 64; do
    echo '#include <math.h>' > inline${arch}.c
    echo '#include "qjs-pink.h"' >> inline${arch}.c
    echo '#include "inline-macros.h"' >> inline${arch}.c
    name=EXTRA_CFLAGS_${arch}
    bear bash -c "${CC} ${CFLAGS} ${!name} -c -o inline${arch}.o inline${arch}.c"
    c2rust transpile --no-simplify-structures --emit-no-std --translate-const-macros --overwrite-existing compile_commands.json
    ./post-process inline${arch}.rs > ../src/inline${arch}.rs
    rustfmt ../src/inline${arch}.rs
    rm -rf inline${arch}.rs inline${arch}.c compile_commands.json
done
