#include "quickjs-ext.h"

int JS_NewGlobalClass(JSContext *ctx, const char *name, int class_id,
                      JSClassDef *class_def, JSCFunction *constructor,
                      int constructor_n_args,
                      const JSCFunctionListEntry *class_funcs,
                      int class_funcs_count,
                      const JSCFunctionListEntry *proto_funcs,
                      int proto_funcs_count) {
    JSValue proto, func_obj;

    JS_NewClass(JS_GetRuntime(ctx), class_id, class_def);
    proto = JS_NewObject(ctx);
    JS_SetClassProto(ctx, class_id, proto);

    if (proto_funcs)
        JS_SetPropertyFunctionList(ctx, proto, proto_funcs, proto_funcs_count);

    func_obj = JS_NewGlobalCConstructor(ctx, name, constructor,
                                        constructor_n_args, proto);
    if (class_funcs)
        JS_SetPropertyFunctionList(ctx, func_obj, class_funcs,
                                   class_funcs_count);
    return 0;
}

int js_set_global_property(JSContext *ctx, const char *prop,
                           JSValueConst value) {
    JSValue global_obj = JS_GetGlobalObject(ctx);
    int ret = JS_SetPropertyStr(ctx, global_obj, prop, value);
    JS_FreeValue(ctx, global_obj);
    return ret;
}

JSValue js_get_global_property(JSContext *ctx, const char *prop) {
    JSValue global_obj = JS_GetGlobalObject(ctx);
    JSValue ret = JS_GetPropertyStr(ctx, global_obj, prop);
    JS_FreeValue(ctx, global_obj);
    return ret;
}
