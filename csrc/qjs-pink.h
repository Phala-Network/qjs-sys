#ifndef __QJS_PINK_H
#define __QJS_PINK_H
#include <stddef.h>
#include "quickjs.h"

typedef void (*output_str_t)(void *userdata, const char *output);
typedef void (*output_bytes_t)(void *userdata, const char *output, int output_len);
typedef int (*input_handler_t)(void *userdata, int i, const char *input, int input_len);
typedef int (*read_args_t)(void *userdata, void *engine_userdata, input_handler_t handler);
typedef struct {
    void *userdata;
    output_str_t output_str;
    output_bytes_t output_bytes;
    read_args_t read_args;
} callbacks_t;
typedef struct {
    const void *code;
    size_t code_len;
    int is_bytecode;
} code_t;

int js_eval(code_t *codes, size_t n_codes, callbacks_t *callbacks);
void js_env_add_helpers(JSContext *ctx);
int js_eval_code(JSContext *ctx, const code_t* code, callbacks_t* callbacks);
void js_std_dump_error(JSContext *ctx);
void js_std_exception(JSContext *ctx, JSValueConst exception_val);
int js_stream_init(JSContext*);

#ifdef CONFIG_BIGNUM
#include "libbf.h"
bf_t *JS_ToBigInt(JSContext *ctx, bf_t *buf, JSValueConst val);
#endif

#endif