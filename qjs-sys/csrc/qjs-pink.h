#ifndef __QJS_PINK_H
#define __QJS_PINK_H
#include <stddef.h>
#include "quickjs.h"
#include "ext/quickjs-ext.h"

typedef void (*output_t)(JSContext *ctx, void *userdata, JSValueConst output);
typedef void (*output_error_t)(JSContext *ctx, void *userdata, const char* err);
typedef int (*input_handler_t)(void *userdata, int i, const char *input, int input_len);
typedef int (*read_args_t)(void *userdata, void *engine_userdata, input_handler_t handler);
typedef struct {
    void *userdata;
    output_t output;
    output_error_t output_err;
    read_args_t read_args;
} callbacks_t;
typedef struct {
    const void *code;
    size_t code_len;
    int is_bytecode;
} code_t;

void js_pink_env_init(JSContext *ctx);
int js_eval_code(JSContext *ctx, const code_t* code, callbacks_t* callbacks);
void js_std_dump_error(JSContext *ctx);
void js_dump_exception(JSContext *ctx, JSValueConst exception_val);

#ifdef CONFIG_BIGNUM
#include "libbf.h"
bf_t *JS_ToBigInt(JSContext *ctx, bf_t *buf, JSValueConst val);
#endif

#endif