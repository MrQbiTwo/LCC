/* C89 integer limits for the signed-char, LP64 target. */
#ifndef __LCC_LIMITS_H
#define __LCC_LIMITS_H

#define CHAR_BIT 8
#define SCHAR_MIN (-127 - 1)
#define SCHAR_MAX 127
#define UCHAR_MAX 255
#define CHAR_MIN SCHAR_MIN
#define CHAR_MAX SCHAR_MAX

/* The implemented C locale uses single-byte characters. */
#define MB_LEN_MAX 1

#define SHRT_MIN (-32767 - 1)
#define SHRT_MAX 32767
#define USHRT_MAX 65535
#define INT_MIN (-2147483647 - 1)
#define INT_MAX 2147483647
#define UINT_MAX 4294967295U
#define LONG_MIN (-9223372036854775807L - 1L)
#define LONG_MAX 9223372036854775807L
#define ULONG_MAX 18446744073709551615UL

#endif
