#include "quickjs-opaque.h"

typedef struct {
    int tag;
    void *data;
    opaque_free_fn free_func;
} OpaqueData;

static OpaqueData *opaque_new(JSContext *ctx, int tag, void *user_data,
                              void (*free_func)(JSRuntime *rt, void *data,
                                                int tag)) {
    OpaqueData *data;

    if (!(data = js_mallocz(ctx, sizeof(OpaqueData))))
        return 0;
    data->tag = tag;
    data->data = user_data;
    data->free_func = free_func;
    return data;
}

static void opaque_free(JSRuntime *rt, OpaqueData *data) {
    if (data->free_func) {
        data->free_func(rt, data->data, data->tag);
    }
    js_free_rt(rt, data);
}

static void opaque_finalizer(JSRuntime *rt, JSValue val) {
    OpaqueData *opaque;

    if ((opaque = JS_GetOpaque(val, JS_CLASS_OPAQUE)))
        opaque_free(rt, opaque);
}

static JSClassDef js_opaque_class = {
    .class_name = "Opaque",
    .finalizer = opaque_finalizer,
};

int js_opaque_class_init(JSContext *ctx) {
    JS_NewClass(JS_GetRuntime(ctx), JS_CLASS_OPAQUE, &js_opaque_class);
    JS_SetClassProto(ctx, JS_CLASS_OPAQUE, JS_NewObject(ctx));
}

JSValue JS_OpaqueObjectNew(JSContext *ctx, void *data, opaque_free_fn free_func,
                           int tag) {
    JSValue obj = JS_NewObjectClass(ctx, JS_CLASS_OPAQUE);
    OpaqueData *opaque = opaque_new(ctx, tag, data, free_func);
    JS_SetOpaque(obj, opaque);
    return obj;
}

void *JS_OpaqueObjectDataGet(JSContext *ctx, JSValueConst obj, int tag) {
    OpaqueData *opaque = JS_GetOpaque(obj, JS_CLASS_OPAQUE);
    if (!opaque || opaque->tag != tag) {
        return 0;
    }
    return opaque->data;
}