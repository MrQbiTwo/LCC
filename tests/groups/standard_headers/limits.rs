// Limits cases for standard headers.
use super::pass;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "limits_character",
            "character limits are usable in preprocessing integer expressions",
            "#if CHAR_BIT != 8 || SCHAR_MIN != -128 || SCHAR_MAX != 127 || UCHAR_MAX != 255 || \
            CHAR_MIN != -128 || CHAR_MAX != 127\n\
            #error character limits\n\
            #endif\n\
            int main(void){\
                return !(sizeof(char)==1&&(char)-1<0);\
            }",
        ),
        (
            "limits_short",
            "short limits match the signed and unsigned sixteen-bit range",
            "#if SHRT_MIN != -32768 || SHRT_MAX != 32767 || USHRT_MAX != 65535\n\
            #error short limits\n\
            #endif\n\
            int main(void){\
                return !((short)SHRT_MIN==-32768&&(unsigned short)USHRT_MAX==65535);\
            }",
        ),
        (
            "limits_int",
            "int limits are preprocessing constants with the full thirty-two-bit range",
            "#if INT_MIN != (-2147483647-1) || INT_MAX != 2147483647 || UINT_MAX != 4294967295U\n\
            #error int limits\n\
            #endif\n\
            int main(void){\
                return !(sizeof(INT_MIN)==4&&sizeof(INT_MAX)==4&&sizeof(UINT_MAX)==4);\
            }",
        ),
        (
            "limits_long",
            "long limits preserve sixty-four-bit boundary values and types",
            "#if LONG_MIN != (-9223372036854775807L-1L) || LONG_MAX != 9223372036854775807L || \
            ULONG_MAX != 18446744073709551615UL\n\
            #error long limits\n\
            #endif\n\
            int main(void){\
                return !(sizeof(LONG_MIN)==8&&sizeof(LONG_MAX)==8&&sizeof(ULONG_MAX)==8&&LONG_MIN<0);\
            }",
        ),
        (
            "limits_promotions",
            "small integer limit macros have their integer promotion types",
            "int main(void){\
                return !(sizeof(SCHAR_MIN)==sizeof(int)&&sizeof(SCHAR_MAX)==sizeof(int)&&sizeof(UCHAR_MAX)==sizeof(int)&&\
                sizeof(SHRT_MIN)==sizeof(int)&&sizeof(SHRT_MAX)==sizeof(int)&&sizeof(USHRT_MAX)==sizeof(int));\
            }",
        ),
        (
            "limits_unsigned",
            "unsigned limit macros retain unsigned arithmetic at their declared rank",
            "int main(void){\
                return !(UINT_MAX+1U==0U&&ULONG_MAX+1UL==0UL&&UINT_MAX>-1L&&ULONG_MAX>-1L-1L);\
            }",
        ),
        (
            "limits_parentheses",
            "negative minima remain single macro expressions under multiplication",
            "int main(void){\
                return !(SCHAR_MIN*0==0&&SHRT_MIN*0==0&&INT_MIN*0==0&&LONG_MIN*0L==0L&&INT_MIN+INT_MAX==-1&&\
                LONG_MIN+LONG_MAX==-1L);\
            }",
        ),
        (
            "limits_c_locale",
            "the implemented C locale uses one-byte multibyte characters",
            "#if MB_LEN_MAX != 1\n#error multibyte limit\n#endif\nint main(void){return 0;}",
        ),
    ] {
        cases.push(pass(id, purpose, "#include <limits.h>", source));
    }
}
