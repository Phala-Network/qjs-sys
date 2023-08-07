#ifndef _QUICKJS_EXT_H
#define _QUICKJS_EXT_H
#include <quickjs.h>
static inline int js_set_global_property(JSContext *ctx, const char *prop, JSValueConst value) {
    JSValue global_obj = JS_GetGlobalObject(ctx);
    int ret = JS_SetPropertyStr(ctx, global_obj, prop, value);
    JS_FreeValue(ctx, global_obj);
    return ret;
}
int js_stream_init(JSContext*);
int js_blob_init(JSContext*);
#endif