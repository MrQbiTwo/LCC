/* Fundamental types and object layout for the Linux x86-64 LP64 target. */
#ifndef __LCC_STDDEF_H
#define __LCC_STDDEF_H
typedef unsigned long size_t;
typedef long ptrdiff_t;
typedef int wchar_t;
#define NULL ((void *)0)
#define offsetof(type, member) __lcc_offsetof(type, member)
#endif
