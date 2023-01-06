#include <assert.h>
#include <inttypes.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "cutils.h"
#include "list.h"
#include "quickjs.h"
#include "qjs-pink.h"


static void js_dump_obj(JSContext *ctx, FILE *f, JSValueConst val) {
    const char *str;

    str = JS_ToCString(ctx, val);
    if (str) {
        fprintf(f, "%s\n", str);
        JS_FreeCString(ctx, str);
    } else {
        fprintf(f, "[exception]\n");
    }
}

static void js_std_dump_error(JSContext *ctx, JSValueConst exception_val) {
    JSValue val;
    BOOL is_error;

    is_error = JS_IsError(ctx, exception_val);
    js_dump_obj(ctx, stderr, exception_val);
    if (is_error) {
        val = JS_GetPropertyStr(ctx, exception_val, "stack");
        if (!JS_IsUndefined(val)) {
            js_dump_obj(ctx, stderr, val);
        }
        JS_FreeValue(ctx, val);
    }
}

static JSValue js_print(JSContext *ctx, JSValueConst this_val, int argc,
                        JSValueConst *argv) {
    int i;
    const char *str;
    size_t len;

    (void)this_val;

    for (i = 0; i < argc; i++) {
        if (i != 0)
            putchar(' ');
        str = JS_ToCStringLen(ctx, &len, argv[i]);
        if (!str)
            return JS_EXCEPTION;
        fwrite(str, 1, len, stdout);
        JS_FreeCString(ctx, str);
    }
    putchar('\n');
    fflush(stdout);
    return JS_UNDEFINED;
}


JSValue __host_call(JSContext *ctx, JSValueConst this_val, int argc, JSValueConst *argv);

static void js_env_add_helpers(JSContext *ctx) {
    JSValue global_obj, console;
    global_obj = JS_GetGlobalObject(ctx);
    console = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, console, "log",
                      JS_NewCFunction(ctx, js_print, "log", 1));
    JS_SetPropertyStr(ctx, console, "error",
                      JS_NewCFunction(ctx, js_print, "error", 1));
    JS_SetPropertyStr(ctx, global_obj, "console", console);
    JS_SetPropertyStr(ctx, global_obj, "print",
                      JS_NewCFunction(ctx, js_print, "print", 1));
    JS_SetPropertyStr(ctx, global_obj, "__hostCall",
                      JS_NewCFunction(ctx, __host_call, "__hostCall", 1));
    JS_FreeValue(ctx, global_obj);
}

struct callback_data {
    JSContext *ctx;
    JSValue args;
};

static int build_args(void *userdata, int i, const char* arg, int len)
{
    int ret;
    struct callback_data *data = (struct callback_data*)userdata;

    ret = JS_DefinePropertyValueUint32(data->ctx, data->args, i,
                                        JS_NewStringLen(data->ctx, arg, len),
                                        JS_PROP_C_W_E);
    if (ret < 0) {
        return -1;
    } else {
        return 0;
    }
}

static int js_env_add_args(JSContext *ctx, callbacks_t* callbacks) {
    JSValue global_obj, args;
    int ret = -1;
    global_obj = JS_GetGlobalObject(ctx);
    args = JS_NewArray(ctx);
    if (!JS_IsException(args)) {
        struct callback_data data = {
            ctx, args
        };
        ret = callbacks->read_args(callbacks->userdata, &data, build_args);
    }
    if (ret < 0) {
        JS_FreeValue(ctx, args);
    } else {
        JS_SetPropertyStr(ctx, global_obj, "scriptArgs", args);
    }
    JS_FreeValue(ctx, global_obj);
    return ret;
}

static JSValue get_output(JSContext *ctx) {
    JSValue global_obj, output;

    global_obj = JS_GetGlobalObject(ctx);
    output = JS_GetPropertyStr(ctx, global_obj, "scriptOutput");
    JS_FreeValue(ctx, global_obj);
    return output;
}

static void put_val(JSContext *ctx, JSValue val, callbacks_t* callbacks) {
    if (JS_IsUint8Array(val)) {
        uint32_t size = 0;
        uint8_t* buffer = JS_Uint8ArrayGetBuffer(val, &size);
        if(buffer != NULL) {
            callbacks->output_bytes(callbacks->userdata, (const char*)buffer, size);
        } else {
        }
    } else {
        const char *str = JS_ToCString(ctx, val);
        if (str == NULL) {
            callbacks->output_str(callbacks->userdata, "<NullValue>");
        } else {
            callbacks->output_str(callbacks->userdata, str);
            JS_FreeCString(ctx, str);
        }
    }
}

static JSValue eval_bytecode(JSContext *ctx, const uint8_t *buf, size_t buf_len) {
    JSValue obj = JS_ReadObject(ctx, buf, buf_len, JS_READ_OBJ_BYTECODE);
    if (JS_IsException(obj))
        return obj;

    return JS_EvalFunction(ctx, obj);
}

static int eval_buf(JSContext *ctx, const void *buf, int buf_len, int is_bytecode, callbacks_t* callbacks) {
    JSValue val;
    int ret;

    if (is_bytecode) {
        val = eval_bytecode(ctx, buf, buf_len);
    } else {
        val = JS_Eval(ctx, buf, buf_len, "<eval>", 0);
    }
    if (JS_IsException(val)) {
        JSValue exception_val = JS_GetException(ctx);
        put_val(ctx, exception_val, callbacks);
        js_std_dump_error(ctx, exception_val);
        JS_FreeValue(ctx, exception_val);
        ret = -1;
    } else {
        JSValue output = get_output(ctx);

        if (!JS_IsUndefined(output)) {
            put_val(ctx, output, callbacks);
        } else {
            put_val(ctx, val, callbacks);
        }
        JS_FreeValue(ctx, output);
        ret = 0;
    }
    JS_FreeValue(ctx, val);
    return ret;
}

int js_eval(const void *code, size_t code_len, int is_bytecode, callbacks_t* callbacks) {
    JSRuntime *rt;
    JSContext *ctx;

    rt = JS_NewRuntime();
    if (!rt) {
        fprintf(stderr, "Failed to create JS runtime\n");
        callbacks->output_str(callbacks->userdata, "<RuntimeCreationError>");
        return 1;
    }

    ctx = JS_NewContext(rt);
    if (!ctx) {
        fprintf(stderr, "Failed to create JS context\n");
        callbacks->output_str(callbacks->userdata, "<ContextCreationError>");
        return 2;
    }

    js_env_add_helpers(ctx);
    if (js_env_add_args(ctx, callbacks) != 0) {
        fprintf(stderr, "Failed to build arguments \n");
        callbacks->output_str(callbacks->userdata, "<InvalidArgs>");
        return 3;
    }

    return eval_buf(ctx, code, code_len, is_bytecode, callbacks);
}
