#ifndef QJS_OPAQUE_H
#define QJS_OPAQUE_H
#include "quickjs.h"

typedef void (*opaque_free_fn)(JSRuntime *rt, void *data, int tag);

int js_opaque_class_init(JSContext *ctx);
JSValue JS_OpaqueObjectNew(
    JSContext *ctx,
    void *data,
    opaque_free_fn free_func,
    JSClassGCMark gc_mark,
    int tag
);
void *JS_OpaqueObjectDataGet(JSValueConst obj, int tag);
void JS_OpaqueObjectDataForget(JSContext *ctx, JSValueConst obj);

#endif