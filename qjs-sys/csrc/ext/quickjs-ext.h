#ifndef _QUICKJS_EXT_H
#define _QUICKJS_EXT_H
#include <quickjs.h>
enum {
    JS_CLASS_BLOB = JS_CLASS_INIT_COUNT,
    JS_CLASS_READABLE_STREAM,
    JS_CLASS_READABLE_STREAM_CONTROLLER,
    JS_CLASS_WRITABLE_STREAM,
    JS_CLASS_WRITABLE_STREAM_CONTROLLER,
    JS_CLASS_STREAM_READER,
    JS_CLASS_STREAM_WRITER,
    JS_CLASS_TRANSFORM_STREAM,
    JS_CLASS_TRANSFORM_STREAM_CONTROLLER,
};
int js_stream_init(JSContext *);
int js_blob_init(JSContext *);
int JS_NewGlobalClass(JSContext *ctx, const char *name, int class_id,
                      JSClassDef *class_def, JSCFunction *constructor,
                      int constructor_n_args,
                      const JSCFunctionListEntry *proto_funcs,
                      int proto_funcs_count,
                      const JSCFunctionListEntry *class_funcs,
                      int class_funcs_count);
int js_set_global_property(JSContext *ctx, const char *prop, JSValueConst value);
JSValue js_get_global_property(JSContext *ctx, const char *prop);
#endif