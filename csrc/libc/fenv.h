#ifndef __FENV_H__
#define __FENV_H__

#ifdef __mips_soft_float
#define FE_ALL_EXCEPT 0
#define FE_TONEAREST  0
#else
#define FE_INEXACT    4
#define FE_UNDERFLOW  8
#define FE_OVERFLOW   16
#define FE_DIVBYZERO  32
#define FE_INVALID    64

#define FE_ALL_EXCEPT 124

#define FE_TONEAREST  0
#define FE_TOWARDZERO 1
#define FE_UPWARD     2
#define FE_DOWNWARD   3
#endif

typedef unsigned short fexcept_t;

typedef struct {
	unsigned __cw;
} fenv_t;

#define FE_DFL_ENV      ((const fenv_t *) -1)

int feclearexcept(int);
int fegetexceptflag(fexcept_t *, int);
int feraiseexcept(int);
int fesetexceptflag(const fexcept_t *, int);
int fetestexcept(int);

int fegetround(void);
int fesetround(int);

int fegetenv(fenv_t *);
int feholdexcept(fenv_t *);
int fesetenv(const fenv_t *);
int feupdateenv(const fenv_t *);

#endif