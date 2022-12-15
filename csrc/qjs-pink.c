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

static void js_std_dump_error(JSContext *ctx, JSValueConst exception_val);

/**********************************************************/

static JSValue js_print(JSContext *ctx, JSValueConst this_val, int argc,
                        JSValueConst *argv) {
    int i;
    const char *str;
    size_t len;

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

void js_env_add_helpers(JSContext *ctx) {
    JSValue global_obj, console, args;
    global_obj = JS_GetGlobalObject(ctx);
    console = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, console, "log",
                      JS_NewCFunction(ctx, js_print, "log", 1));
    JS_SetPropertyStr(ctx, console, "error",
                      JS_NewCFunction(ctx, js_print, "error", 1));
    JS_SetPropertyStr(ctx, global_obj, "console", console);
    JS_SetPropertyStr(ctx, global_obj, "print",
                      JS_NewCFunction(ctx, js_print, "print", 1));
    JS_FreeValue(ctx, global_obj);
}

static void put_val(JSContext *ctx, JSValue val, void *userdata,
                    callback_t callback) {
    const char *str = JS_ToCString(ctx, val);
    if (str == NULL) {
        callback(userdata, "<NullValue>");
    } else {
        callback(userdata, str);
        JS_FreeCString(ctx, str);
    }
}

static int eval_buf(JSContext *ctx, const void *buf, int buf_len,
                    void *userdata, callback_t callback) {
    JSValue val;
    int ret;

    val = JS_Eval(ctx, buf, buf_len, "<qjs>", 0);
    if (JS_IsException(val)) {
        JSValue exception_val = JS_GetException(ctx);
        put_val(ctx, exception_val, userdata, callback);
        js_std_dump_error(ctx, exception_val);
        JS_FreeValue(ctx, exception_val);
        ret = -1;
    } else {
        ret = 0;
        put_val(ctx, val, userdata, callback);
    }
    JS_FreeValue(ctx, val);
    return ret;
}

int js_evaluate(const void *buf, size_t buf_len, void *userdata,
                callback_t callback) {
    JSRuntime *rt;
    JSContext *ctx;

    rt = JS_NewRuntime();
    if (!rt) {
        fprintf(stderr, "Failed to create JS runtime\n");
        callback(userdata, "<RuntimeCreationError>");
        return 2;
    }

    ctx = JS_NewContext(rt);
    if (!ctx) {
        fprintf(stderr, "Failed to create JS context\n");
        callback(userdata, "<ContextCreationError>");
        return 2;
    }

    js_env_add_helpers(ctx);

    return eval_buf(ctx, buf, buf_len, userdata, callback);
}

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
