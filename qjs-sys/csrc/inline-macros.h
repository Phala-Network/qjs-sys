#ifndef _INLINE_MACROS_H
#define _INLINE_MACROS_H
#include "quickjs.h"

EXPORT static inline int64_t JS_GetTag(JSValueConst v) {
    return JS_VALUE_GET_TAG(v);
}

EXPORT static inline int32_t JS_GetInt(JSValueConst v) {
    return JS_VALUE_GET_INT(v);
}

EXPORT static inline int32_t JS_GetBool(JSValueConst v) {
    return JS_VALUE_GET_BOOL(v);
}

EXPORT static inline double JS_GetFloat64(JSValueConst v) {
    return JS_VALUE_GET_FLOAT64(v);
}

EXPORT static inline void *JS_GetPtr(JSValueConst v) {
    return JS_VALUE_GET_PTR(v);
}

EXPORT static inline JSValue JS_MakeValue(int32_t tag, int32_t val) {
    return JS_MKVAL(tag, val);
}

EXPORT static inline JSValue JS_MakePtr(int32_t tag, uintptr_t p) {
    return JS_MKPTR(tag, p);
}

EXPORT static inline JSValue JS_MakeNAN() { return JS_NAN; }

EXPORT static inline JSValue JS_MakeNULL() { return JS_NULL; }

EXPORT static inline JSValue JS_MakeUNDEFINED() { return JS_UNDEFINED; }

EXPORT static inline JSValue JS_MakeFALSE() { return JS_FALSE; }

EXPORT static inline JSValue JS_MakeTRUE() { return JS_TRUE; }

EXPORT static inline JSValue JS_MakeEXCEPTION() { return JS_EXCEPTION; }

EXPORT static inline JSValue JS_MakeUNINITIALIZED() { return JS_UNINITIALIZED; }

typedef enum {
    __JS_ATOM_NULL = JS_ATOM_NULL,
#define DEF(name, str) JS_ATOM_ ## name,
#include "quickjs-atom.h"
#undef DEF
    JS_ATOM_END,
} JSAtomEnum;

EXPORT static inline int _to_keep_symbols(int flags) {
    JSAtomEnum a = JS_ATOM_END;
    return (a & flags & JS_PROP_CONFIGURABLE & JS_PROP_WRITABLE &
                JS_PROP_ENUMERABLE &&
            JS_PROP_LENGTH & JS_PROP_TMASK & JS_PROP_NORMAL & JS_PROP_GETSET &
                JS_PROP_VARREF & JS_PROP_AUTOINIT);
}


#endif

