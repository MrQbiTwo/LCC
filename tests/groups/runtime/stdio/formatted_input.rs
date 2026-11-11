// Formatted input cases for runtime/stdio.
use super::io;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, body, input, expected) in [
        (
            "scan_integer",
            "signed conversion leaves newline for getchar",
            "int n,x=99,c;n=scanf(\"%d\",&x);c=getchar();printf(\"%d %d %d\",n,x,c);return 0;",
            "-2147483648\n",
            "1 -2147483648 10",
        ),
        (
            "scan_radices",
            "i recognizes octal and hexadecimal while d stays decimal",
            "int n,a,b,c;\
            n=scanf(\"%i%i%d\",&a,&b,&c);\
            printf(\"%d %d %d %d %d\",n,a,b,c,getchar());\
            return 0;",
            "017 0x7f 09!",
            "3 15 127 9 33",
        ),
        (
            "scan_unsigned",
            "unsigned and long destinations accept their full range",
            "unsigned u;\
            unsigned short s;\
            unsigned long l;\
            int n;\
            n=scanf(\"%u%hu%lu\",&u,&s,&l);\
            printf(\"%d %u %hu %lu\",n,u,s,l);\
            return 0;",
            "4294967295 65535 18446744073709551615",
            "3 4294967295 65535 18446744073709551615",
        ),
        (
            "scan_signed_lengths",
            "short and long input write the declared width",
            "struct S{\
                short x;\
                short guard;\
                long y;\
            };\
            struct S s={\
                0,123,0};\
            int n;\
            n=scanf(\"%hd%ld\",&s.x,&s.y);\
            printf(\"%d %hd %hd %ld\",n,s.x,s.guard,s.y);\
            return 0;",
            "-32768 -9223372036854775808",
            "2 -32768 123 -9223372036854775808",
        ),
        (
            "scan_float_double",
            "float and double input write distinct sizes and exact values",
            "float f;\
            double d;\
            int n;\
            n=scanf(\"%f%lf\",&f,&d);\
            printf(\"%d %.27f %.2f %d\",n,f,d,getchar());\
            return 0;",
            "0.1 -2.25!",
            "2 0.100000001490116119384765625 -2.25 33",
        ),
        (
            "scan_exponent",
            "decimal input accepts exponent signs and fractional shorthand",
            "double a,b,c;\
            int n;\
            n=scanf(\"%lf%lf%lf\",&a,&b,&c);\
            printf(\"%d %.3f %.2f %.0f\",n,a,b,c);\
            return 0;",
            "1e-3 -.5 12.",
            "3 0.001 -0.50 12",
        ),
        (
            "scan_special",
            "case-insensitive infinity and NaN spellings round-trip",
            "double a,b,c;int n;n=scanf(\"%lf%lf%lf\",&a,&b,&c);printf(\"%d %f %f %f\",n,a,b,c);return 0;",
            "+InFiNiTy -INF NaN",
            "3 inf -inf nan",
        ),
        (
            "scan_integer_width",
            "field widths stop before the next conversion's input",
            "int a,b,n;n=scanf(\"%2d%2d\",&a,&b);printf(\"%d %d %d %d\",n,a,b,getchar());return 0;",
            "12345",
            "2 12 34 53",
        ),
        (
            "scan_float_width",
            "floating field width excludes the following byte",
            "double d;int n;n=scanf(\"%3lf\",&d);printf(\"%d %.1f %d\",n,d,getchar());return 0;",
            "5.67",
            "1 5.6 55",
        ),
        (
            "scan_string",
            "string scanning skips whitespace, applies width and appends zero",
            "char s[4];int n,c;n=scanf(\"%3s\",s);c=getchar();printf(\"%d %s %d\",n,s,c);return 0;",
            " \t abcdef",
            "1 abc 100",
        ),
        (
            "scan_chars",
            "c consumes whitespace and does not append a terminator",
            "char s[4]={\
                '?','?','?','!'};\
            int n;\
            n=scanf(\"%3c\",s);\
            printf(\"%d %d %d %d %d %d\",n,s[0],s[1],s[2],s[3],getchar());\
            return 0;",
            "a bQ",
            "1 97 32 98 33 81",
        ),
        (
            "scan_char_space",
            "format whitespace before c skips input whitespace",
            "char c;int n;n=scanf(\" %c\",&c);printf(\"%d %d\",n,c);return 0;",
            "\r\n\t Q",
            "1 81",
        ),
        (
            "scan_suppression",
            "suppression consumes input without advancing destination arguments",
            "short a;\
            int n;\
            double b;\
            char s[4];\
            n=scanf(\"%*d%hd%*lf%lf%*c%3s\",&a,&b,s);\
            printf(\"%d %hd %.2f %s\",n,a,b,s);\
            return 0;",
            "7 -12 2.5 1.25Q abc!",
            "3 -12 1.25 abc",
        ),
        (
            "scan_literal",
            "literal characters and percent match input exactly",
            "int a=9,n;n=scanf(\" x %% %d\",&a);printf(\"%d %d %d\",n,a,getchar());return 0;",
            " \n x % 42!",
            "1 42 33",
        ),
        (
            "scan_literal_mismatch",
            "literal mismatch preserves the unrelated byte",
            "int a=9,n;n=scanf(\"x%%%d\",&a);printf(\"%d %d %d\",n,a,getchar());return 0;",
            "xQ42",
            "0 9 81",
        ),
        (
            "scan_mismatch",
            "numeric mismatch preserves the destination and first unrelated byte",
            "int a=9,n;n=scanf(\"%d\",&a);printf(\"%d %d %d\",n,a,getchar());return 0;",
            "x!",
            "0 9 120",
        ),
        (
            "scan_assignment_count",
            "matching failure retains the number of earlier assignments",
            "int a=9,b=8,n;n=scanf(\"%d%d\",&a,&b);printf(\"%d %d %d %d\",n,a,b,getchar());return 0;",
            "7 x",
            "1 7 8 120",
        ),
        (
            "scan_eof",
            "EOF before any assignment returns EOF and preserves destination",
            "int a=9,n;n=scanf(\"%d\",&a);printf(\"%d %d %d\",n,a,getchar());return 0;",
            "",
            "-1 9 -1",
        ),
        (
            "scan_space_eof",
            "whitespace exhaustion is an input failure",
            "int a=9,n;n=scanf(\"%d\",&a);printf(\"%d %d\",n,a);return 0;",
            " \r\n\t",
            "-1 9",
        ),
        (
            "scan_partial_eof",
            "EOF after an assignment returns the successful count",
            "int a=9,b=8,n;n=scanf(\"%d%d\",&a,&b);printf(\"%d %d %d\",n,a,b);return 0;",
            "7",
            "1 7 8",
        ),
        (
            "scan_suppressed_eof",
            "a suppressed conversion is not counted as assignment",
            "int a=9,n;n=scanf(\"%*d%d\",&a);printf(\"%d %d\",n,a);return 0;",
            "7",
            "-1 9",
        ),
        (
            "scan_empty",
            "empty format succeeds without input consumption",
            "int n;n=scanf(\"\");printf(\"%d %d\",n,getchar());return 0;",
            "Q",
            "0 81",
        ),
        (
            "scan_overflow_arguments",
            "destination pointer arguments continue onto stack",
            "int a,b,c,d,e,f,g,h,n;\
            n=scanf(\"%d%d%d%d%d%d%d%d\",&a,&b,&c,&d,&e,&f,&g,&h);\
            printf(\"%d %d %d %d %d %d %d %d %d\",n,a,b,c,d,e,f,g,h);\
            return 0;",
            "1 2 3 4 5 6 7 8",
            "8 1 2 3 4 5 6 7 8",
        ),
        (
            "shared_cursor",
            "getchar and repeated scanf calls share one buffered cursor",
            "int a,b,c,n,m;\
            char s;\
            n=scanf(\"%d\",&a);\
            c=getchar();\
            m=scanf(\"%d%c\",&b,&s);\
            printf(\"%d %d %d %d %d %d %d\",n,a,c,m,b,s,getchar());\
            return 0;",
            "12,34Z!",
            "1 12 44 2 34 90 33",
        ),
        (
            "getchar_bytes",
            "getchar returns unsigned bytes and stable EOF",
            "int a,b,c,d;\
            a=getchar();\
            b=getchar();\
            c=getchar();\
            d=getchar();\
            printf(\"%d %d %d %d\",a,b,c,d);\
            return 0;",
            "\u{00ff}",
            "195 191 -1 -1",
        ),
        (
            "prompt",
            "a prompt without newline is flushed before a blocking input read",
            "int c;printf(\"prompt>\");c=getchar();printf(\"%d\",c);return 0;",
            "Q",
            "prompt>81",
        ),
    ] {
        cases.push(io(id, purpose, body, input, expected.into()));
    }
}
