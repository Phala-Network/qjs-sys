#ifndef __QJS_PINK_H
#define __QJS_PINK_H
#include <stddef.h>

typedef void (*callback_t)(void *userdata, const char *output);

int js_evaluate(const void *buf, size_t buf_len, void *userdata,
                callback_t callback, int binary);
#endif