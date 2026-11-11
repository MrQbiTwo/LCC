/* System V variable arguments use compiler builtins and the native list layout. */
#ifndef __LCC_STDARG_H
#define __LCC_STDARG_H
typedef struct __lcc_va_state {
    unsigned int gp_offset;
    unsigned int fp_offset;
    void *overflow_arg_area;
    void *reg_save_area;
} va_list[1];
#define va_start(ap, last) __lcc_va_start(ap, last)
#define va_arg(ap, type) __lcc_va_arg(ap, type)
#define va_end(ap) __lcc_va_end(ap)
/* Independent cursor copying is supported as an extension to C89. */
#define va_copy(dst, src) __lcc_va_copy(dst, src)
#endif
