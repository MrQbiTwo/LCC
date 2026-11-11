/* The implemented standard input/output subset uses the C locale. */
#ifndef __LCC_STDIO_H
#define __LCC_STDIO_H
#define EOF (-1)
int printf(const char *format, ...);
int scanf(const char *format, ...);
int getchar(void);
#endif
