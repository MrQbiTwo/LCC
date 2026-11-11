// Formatted output cases for runtime/stdio.
use super::io;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, body, expected) in [
        (
            "literal",
            "normal exit flushes a final literal without newline",
            "return printf(\"hello\")==5?0:1;",
            "hello",
        ),
        (
            "percent",
            "literal percent does not consume a variadic argument",
            "return printf(\"%% %d %%\",7)==5?0:1;",
            "% 7 %",
        ),
        (
            "signed",
            "signed integer output handles zero and minimum values",
            "printf(\"%d %i %hd %ld\",0,-2147483647-1,-32768,-9223372036854775807L-1L);return 0;",
            "0 -2147483648 -32768 -9223372036854775808",
        ),
        (
            "unsigned",
            "unsigned integer output retains all destination bits",
            "printf(\"%u %hu %lu\",4294967295U,65535U,18446744073709551615UL);return 0;",
            "4294967295 65535 18446744073709551615",
        ),
        (
            "integer_fields",
            "integer flags, precision and alignment interact correctly",
            "printf(\"[%+08d][%-6d][% 5d][%.5d][%.0d]\",-23,7,7,23,0);return 0;",
            "[-0000023][7     ][    7][00023][]",
        ),
        (
            "integer_precision",
            "integer precision overrides zero fill and preserves explicit plus",
            "printf(\"[%08.4d][%+.0d][%8.0u]\",23,0,0U);return 0;",
            "[    0023][+][        ]",
        ),
        (
            "dynamic_fields",
            "negative dynamic width left-aligns and precision remains ordered",
            "printf(\"[%*.*d][%*.*f]\",-8,4,23,9,2,1.25);return 0;",
            "[0023    ][     1.25]",
        ),
        (
            "negative_precision",
            "negative dynamic precision restores the conversion default",
            "printf(\"[%.*f][%.*s][%.*d]\",-1,1.25,-3,\"abc\",-1,7);return 0;",
            "[1.250000][abc][7]",
        ),
        (
            "characters",
            "character and string width and string precision",
            "printf(\"[%3c][%-3c][%7.3s][%-5s][%.0s]\",'A','B',\"abcdef\",\"hi\",\"ignored\");return 0;",
            "[  A][B  ][    abc][hi   ][]",
        ),
        (
            "float_rounding",
            "fixed decimal uses nearest-even ties and decimal carry",
            "printf(\"%.0f %.0f %.2f %.2f %.2f\",0.5,1.5,1.125,1.375,9.999);return 0;",
            "0 2 1.12 1.38 10.00",
        ),
        (
            "float_sign",
            "negative zero and float sign padding remain visible",
            "printf(\"[%+.2f][% .1f][%08.2f][%-8.2f]\",-0.0,1.5,-1.25,1.25);return 0;",
            "[-0.00][ 1.5][-0001.25][1.25    ]",
        ),
        (
            "float_exact",
            "decimal output exposes the exact stored binary64 value",
            "printf(\"%.55f\",0.1);return 0;",
            "0.1000000000000000055511151231257827021181583404541015625",
        ),
        (
            "float_promotion",
            "promoted binary32 retains its exact value at printf",
            "float f=0.1f;printf(\"%.27f\",f);return 0;",
            "0.100000001490116119384765625",
        ),
        (
            "float_l_modifier",
            "the calculator's lf output extension takes a promoted double",
            "printf(\"%.2lf %lf\",1.25,2.5);return 0;",
            "1.25 2.500000",
        ),
        (
            "gp_overflow",
            "integer variadic arguments continue from registers onto stack",
            "printf(\"%d,%d,%d,%d,%d,%d,%d,%d\",1,2,3,4,5,6,7,8);return 0;",
            "1,2,3,4,5,6,7,8",
        ),
        (
            "fp_overflow",
            "floating variadic arguments continue beyond eight XMM registers",
            "printf(\"%.1f,%.1f,%.1f,%.1f,%.1f,%.1f,%.1f,%.1f,%.1f,%.1f\",1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0);\
            return 0;",
            "1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0",
        ),
        (
            "mixed_overflow",
            "interleaved integer and floating stack arguments keep their cursors",
            "printf(\"%d:%.0f,%d:%.0f,%d:%.0f,%d:%.0f,%d:%.0f,%d:%.0f,%d:%.0f,%d:%.0f,%d:%.0f\",1,11.0,2,12.0,3,13.0,4,14.0,5,15.0,6,16.0,7,17.0,8,18.0,9,19.0);\
            return 0;",
            "1:11,2:12,3:13,4:14,5:15,6:16,7:17,8:18,9:19",
        ),
        (
            "empty_output",
            "an empty format returns zero",
            "return printf(\"\")!=0;",
            "",
        ),
        (
            "nul_character",
            "the output length includes a zero byte",
            "return printf(\"A%cB\",0)==3?0:1;",
            "A\0B",
        ),
    ] {
        cases.push(io(id, purpose, body, "", expected.into()));
    }
    cases.push(io(
        "float_subnormal",
        "smallest binary64 subnormal rounds at precision 324",
        "printf(\"%.324f\",4.9406564584124654e-324);return 0;",
        "",
        format!("0.{}5", "0".repeat(323)),
    ));
    cases.push(io(
        "float_maximum",
        "largest finite binary64 formats without integer narrowing",
        "printf(\"%.0f\",1.7976931348623157e308);return 0;",
        "",
        format!("{:.0}", f64::MAX),
    ));
    cases.push(io(
        "precision_limit",
        "supported maximum precision crosses output buffer capacity",
        "return printf(\"%.4096f\",1.0)==4098?0:1;",
        "",
        format!("1.{}", "0".repeat(4096)),
    ));
    cases.push(io(
        "width_limit",
        "supported maximum width returns the full emitted byte count",
        "return printf(\"%4096d\",7)==4096?0:1;",
        "",
        format!("{}7", " ".repeat(4095)),
    ));
    cases.push(io(
        "output_capacity",
        "repeated writes cross multiple buffers and flush the final suffix",
        "int i;for(i=0;i<9000;i++)printf(\"x\");printf(\"end\");return 0;",
        "",
        format!("{}end", "x".repeat(9000)),
    ));
}
