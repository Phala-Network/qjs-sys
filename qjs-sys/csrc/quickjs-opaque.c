#include "quickjs-opaque.h"

typedef struct {
    int tag;
    void *data;
    opaque_free_fn free_func;
    JSClassGCMark *gc_mark;
} OpaqueData;

static OpaqueData *opaque_new(
    JSContext *ctx,
    int tag,
    void *user_data,
    opaque_free_fn free_func,
    JSClassGCMark gc_mark
) {
    OpaqueData *data;

    if (!(data = js_mallocz(ctx, sizeof(OpaqueData))))
        return 0;
    data->tag = tag;
    data->data = user_data;
    data->free_func = free_func;
    data->gc_mark = gc_mark;
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

static void opaque_gc_mark(JSRuntime *rt, JSValue val,
                           JS_MarkFunc *mark_func) {
    OpaqueData *opaque;
    if ((opaque = JS_GetOpaque(val, JS_CLASS_OPAQUE))) {
        if (opaque->gc_mark) {
            opaque->gc_mark(rt, val, mark_func);
        }
    }
}

static JSClassDef js_opaque_class = {
    .class_name = "Opaque",
    .finalizer = opaque_finalizer,
    .gc_mark = opaque_gc_mark,
};

int js_opaque_class_init(JSContext *ctx) {
    JS_NewClass(JS_GetRuntime(ctx), JS_CLASS_OPAQUE, &js_opaque_class);
    JS_SetClassProto(ctx, JS_CLASS_OPAQUE, JS_NewObject(ctx));
    return 0;
}

JSValue JS_OpaqueObjectNew(
    JSContext *ctx,
    void *data,
    opaque_free_fn free_func,
    JSClassGCMark gc_mark,
    int tag
) {
    JSValue obj = JS_NewObjectClass(ctx, JS_CLASS_OPAQUE);
    OpaqueData *opaque = opaque_new(ctx, tag, data, free_func, gc_mark);
    JS_SetOpaque(obj, opaque);
    return obj;
}

void *JS_OpaqueObjectDataGet(JSValueConst obj, int tag) {
    OpaqueData *opaque = JS_GetOpaque(obj, JS_CLASS_OPAQUE);
    if (!opaque || opaque->tag != tag) {
        return 0;
    }
    return opaque->data;
}

void JS_OpaqueObjectDataForget(JSContext *ctx, JSValueConst obj) {
    OpaqueData *opaque = JS_GetOpaque(obj, JS_CLASS_OPAQUE);
    if (!opaque) {
        return;
    }
    JS_SetOpaque(obj, 0);
    js_free_rt(JS_GetRuntime(ctx), opaque);
}