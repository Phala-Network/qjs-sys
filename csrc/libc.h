
#ifndef LIB_C_H
#define LIB_C_H
#include <stddef.h>
#include <stdarg.h>

#include "inttypes-wasm32.h"

#define _Addr int
#define _Int64 long long
#define _Reg int

typedef __builtin_va_list va_list;
typedef __builtin_va_list __isoc_va_list;

#ifndef __cplusplus
typedef int wchar_t;
#endif
typedef unsigned wint_t;

typedef int blksize_t;
typedef unsigned int nlink_t;

typedef unsigned _Addr uintptr_t;
typedef _Addr intptr_t;
typedef float float_t;
typedef double double_t;

typedef long long time_t;
typedef long long suseconds_t;

typedef _Addr ssize_t;
typedef _Addr regoff_t;
typedef _Reg register_t;

typedef signed char     int8_t;
typedef signed short    int16_t;
typedef signed int      int32_t;
typedef signed _Int64   int64_t;
typedef signed _Int64   intmax_t;
typedef unsigned char   uint8_t;
typedef unsigned short  uint16_t;
typedef unsigned int    uint32_t;
typedef unsigned _Int64 uint64_t;
typedef unsigned _Int64 u_int64_t;
typedef unsigned _Int64 uintmax_t;

typedef unsigned mode_t;
typedef unsigned _Reg nlink_t;
typedef _Int64 off_t;
typedef unsigned _Int64 ino_t;
typedef unsigned _Int64 dev_t;
typedef _Int64 blkcnt_t;
typedef unsigned _Int64 fsblkcnt_t;
typedef unsigned _Int64 fsfilcnt_t;

typedef unsigned wint_t;
typedef unsigned long wctype_t;

typedef void * timer_t;
typedef int clockid_t;
typedef long clock_t;
struct timeval { time_t tv_sec; suseconds_t tv_usec; };
struct timespec { time_t tv_sec; long tv_nsec; };

typedef int pid_t;
typedef unsigned id_t;
typedef unsigned uid_t;
typedef unsigned gid_t;
typedef int key_t;
typedef unsigned useconds_t;

#define NULL ((void*)0)

#undef _Addr
#undef _Int64
#undef _Reg


static inline void assert(int b) {}
static inline int putchar(int ch) { return 0; }
void *alloca(size_t);

#define alloca __builtin_alloca

int puts(const char *);

typedef struct FILE FILE;
extern FILE* stderr;

int printf(const char *__restrict, ...);
int fprintf(FILE *__restrict, const char *__restrict, ...);
int sprintf(char *__restrict, const char *__restrict, ...);
int snprintf(char *__restrict, size_t, const char *__restrict, ...);

int vprintf(const char *__restrict, __isoc_va_list);
int vfprintf(FILE *__restrict, const char *__restrict, __isoc_va_list);
int vsprintf(char *__restrict, const char *__restrict, __isoc_va_list);
int vsnprintf(char *__restrict, size_t, const char *__restrict, __isoc_va_list);

int scanf(const char *__restrict, ...);
int sscanf(const char *__restrict, const char *__restrict, ...);
int vscanf(const char *__restrict, __isoc_va_list);
int vfscanf(FILE *__restrict, const char *__restrict, __isoc_va_list);
int vsscanf(const char *__restrict, const char *__restrict, __isoc_va_list);

void perror(const char *);

#endif