#ifndef _INLINE_MACROS_H
#define _INLINE_MACROS_H
#include "quickjs.h"

static inline int64_t JS_GetTag(JSValueConst v) {
    return JS_VALUE_GET_TAG(v);
}

static inline int32_t JS_GetInt(JSValueConst v) {
    return JS_VALUE_GET_INT(v);
}

static inline int32_t JS_GetBool(JSValueConst v) {
    return JS_VALUE_GET_BOOL(v);
}

static inline double JS_GetFloat64(JSValueConst v) {
    return JS_VALUE_GET_FLOAT64(v);
}

static inline void* JS_GetPtr(JSValueConst v) {
    return JS_VALUE_GET_PTR(v);
}

static inline JSValue JS_MakeValue(int32_t tag, int32_t val) {
    return JS_MKVAL(tag, val);
}

static inline JSValue JS_MakePtr(int32_t tag, uintptr_t p) {
    return JS_MKPTR(tag, p);
}

static inline JSValue JS_MakeNAN() {
    return JS_NAN;
}

static inline JSValue JS_MakeNULL() {
    return JS_NULL;
}

static inline JSValue JS_MakeUNDEFINED() {
    return JS_UNDEFINED;
}

static inline JSValue JS_MakeFALSE() {
    return JS_FALSE;
}

static inline JSValue JS_MakeTRUE() {
    return JS_TRUE;
}

static inline JSValue JS_MakeEXCEPTION() {
    return JS_EXCEPTION;
}

static inline JSValue JS_MakeUNINITIALIZED() {
    return JS_UNINITIALIZED;
}

#endif