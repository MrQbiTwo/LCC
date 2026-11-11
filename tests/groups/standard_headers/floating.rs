// Floating cases for standard headers.
use super::pass;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "float_characteristics",
            "binary32 characteristics are preprocessing integer constants",
            "#if FLT_RADIX != 2 || FLT_MANT_DIG != 24 || FLT_DIG != 6 || FLT_MIN_EXP != -125 || \
            FLT_MAX_EXP != 128 || FLT_MIN_10_EXP != -37 || FLT_MAX_10_EXP != 38\n\
            #error float characteristics\n\
            #endif\n\
            int main(void){\
                return 0;\
            }",
        ),
        (
            "double_characteristics",
            "binary64 characteristics are preprocessing integer constants",
            "#if DBL_MANT_DIG != 53 || DBL_DIG != 15 || DBL_MIN_EXP != -1021 || DBL_MAX_EXP != 1024 || \
            DBL_MIN_10_EXP != -307 || DBL_MAX_10_EXP != 308\n\
            #error double characteristics\n\
            #endif\n\
            int main(void){\
                return 0;\
            }",
        ),
        (
            "extended_characteristics",
            "binary80 characteristics describe the padded long double representation",
            "#if LDBL_MANT_DIG != 64 || LDBL_DIG != 18 || LDBL_MIN_EXP != -16381 || LDBL_MAX_EXP != 16384 || \
            LDBL_MIN_10_EXP != -4931 || LDBL_MAX_10_EXP != 4932\n\
            #error extended characteristics\n\
            #endif\n\
            int main(void){\
                return sizeof(long double)!=16;\
            }",
        ),
        (
            "rounding_policy",
            "FLT_ROUNDS reports indeterminable caller-controlled rounding modes",
            "#if FLT_ROUNDS != -1\n#error rounding policy\n#endif\nint main(void){return FLT_ROUNDS*0;}",
        ),
        (
            "float_constant_types",
            "float limits have float type rather than implicit double type",
            "int main(void){return !(sizeof(FLT_MAX)==4&&sizeof(FLT_MIN)==4&&sizeof(FLT_EPSILON)==4);}",
        ),
        (
            "double_constant_types",
            "double limits retain double type",
            "int main(void){return !(sizeof(DBL_MAX)==8&&sizeof(DBL_MIN)==8&&sizeof(DBL_EPSILON)==8);}",
        ),
        (
            "extended_constant_types",
            "long double limits retain long double type",
            "int main(void){\
                return !(sizeof(LDBL_MAX)==16&&sizeof(LDBL_MIN)==16&&sizeof(LDBL_EPSILON)==16);\
            }",
        ),
    ] {
        cases.push(pass(id, purpose, "#include <float.h>", source));
    }
    // Inspect target representations instead of repeating the header's decimal literals.
    for (id, value, bits) in [
        ("float_maximum", "FLT_MAX", "0x7f7fffffU"),
        ("float_minimum", "FLT_MIN", "0x00800000U"),
        ("float_epsilon", "FLT_EPSILON", "0x34000000U"),
    ] {
        cases.push(pass(
            id,
            "binary32 limit has the independently specified IEEE representation",
            "#include <float.h>",
            &format!(
                "union U{{\
                    float f;\
                    unsigned int u;\
                }};\
                int main(void){{\
                    union U v;\
                    v.f={value};\
                    return v.u!={bits};\
                }}"
            ),
        ));
    }
    for (id, value, bits) in [
        ("double_maximum", "DBL_MAX", "0x7fefffffffffffffUL"),
        ("double_minimum", "DBL_MIN", "0x0010000000000000UL"),
        ("double_epsilon", "DBL_EPSILON", "0x3cb0000000000000UL"),
    ] {
        cases.push(pass(
            id,
            "binary64 limit has the independently specified IEEE representation",
            "#include <float.h>",
            &format!(
                "union U{{\
                    double f;\
                    unsigned long u;\
                }};\
                int main(void){{\
                    union U v;\
                    v.f={value};\
                    return v.u!={bits};\
                }}"
            ),
        ));
    }
    for (id, value, significand, exponent) in [
        (
            "extended_maximum",
            "LDBL_MAX",
            "0xffffffffffffffffUL",
            "0x7ffe",
        ),
        ("extended_minimum", "LDBL_MIN", "0x8000000000000000UL", "1"),
        (
            "extended_epsilon",
            "LDBL_EPSILON",
            "0x8000000000000000UL",
            "0x3fc0",
        ),
    ] {
        cases.push(pass(
            id,
            "binary80 limit has the independently specified significand and exponent",
            "#include <float.h>",
            &format!(
                "union U{{\
                    long double f;\
                    struct{{\
                        unsigned long low;\
                        unsigned short high;\
                    }} \
                    bits;\
                }};\
                int main(void){{\
                    union U v;\
                    v.f={value};\
                    return !(v.bits.low=={significand}&&v.bits.high=={exponent});\
                }}"
            ),
        ));
    }
    cases.push(pass(
        "repeat_includes",
        "headers can be repeated in mixed order without redefining declarations",
        "#include <float.h>\n#include <stddef.h>\n#include <limits.h>\n#include <stddef.h>\n#include <float.h>\n#include <limits.h>",
        "struct S{\
            char c;\
            long n;\
        };\
        int main(void){\
            size_t n=offsetof(struct S,n);\
            ptrdiff_t d=-1;\
            wchar_t c=CHAR_MAX;\
            return !(n==8&&d==-1&&c==127&&FLT_RADIX==2);\
        }",
    ));
}
