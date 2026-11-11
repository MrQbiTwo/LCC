// Preprocessing cases separate expansion, translation phases, and source origins.
use crate::{Case, Expectation};

// Each source has an independently reviewed integer result or diagnostic code.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let inputs = [
        (
            "object replacement",
            "#define VALUE 17\nint main(void){return VALUE;}",
            17,
        ),
        (
            "replacement rescanning",
            "#define A B\n#define B 23\nint main(void){return A;}",
            23,
        ),
        (
            "definition order",
            "#define X 4\nint f(void){return X;}\n#undef X\n#define X 9\nint main(void){return f()+X;}",
            13,
        ),
        (
            "empty replacement",
            "#define EMPTY\nint main(EMPTY void){return 6;}",
            6,
        ),
        (
            "function replacement",
            "#define ADD(a,b) ((a)+(b))\nint main(void){return ADD(4,7);}",
            11,
        ),
        (
            "nested function arguments",
            "#define TWICE(x) ((x)+(x))\nint main(void){return TWICE(TWICE(3));}",
            12,
        ),
        (
            "parenthesized argument comma",
            "#define PICK(x,y) (x)\nint main(void){return PICK((3,7),9);}",
            7,
        ),
        (
            "unused argument",
            "#define FIRST(a,b) a\nint main(void){return FIRST(13,undefined_identifier);}",
            13,
        ),
        (
            "multiline invocation",
            "#define ADD(a,b) ((a)+(b))\nint main(void){return ADD(\n4,\n9);}",
            13,
        ),
        (
            "zero parameter macro",
            "#define ANSWER() 42\nint main(void){return ANSWER();}",
            42,
        ),
        (
            "empty parameter",
            "#define MAYBE(x) x\nint main(void){return 5 MAYBE();}",
            5,
        ),
        (
            "object to function rescan",
            "#define A F\n#define F(x) ((x)+2)\nint main(void){return A(7);}",
            9,
        ),
        (
            "function name without invocation",
            "#define name(x) (x)\nint name=11;int main(void){return name;}",
            11,
        ),
        (
            "self replacement suppression",
            "#define x x\nint x=14;int main(void){return x;}",
            14,
        ),
        (
            "indirect replacement suppression",
            "#define a b\n#define b a\nint a=15;int main(void){return a;}",
            15,
        ),
        (
            "paste identifier",
            "#define CAT(a,b) a##b\nint main(void){int value=19;return CAT(val,ue);}",
            19,
        ),
        (
            "paste numeric token",
            "#define CAT(a,b) a##b\nint main(void){return CAT(1,7);}",
            17,
        ),
        (
            "paste raw argument",
            "#define X 5\n#define CAT(a,b) a##b\nint X2=21;int main(void){return CAT(X,2);}",
            21,
        ),
        (
            "paste empty right",
            "#define CAT(a,b) a##b\nint main(void){return CAT(23,);}",
            23,
        ),
        (
            "paste empty left",
            "#define CAT(a,b) a##b\nint main(void){return CAT(,24);}",
            24,
        ),
        (
            "paste both empty",
            "#define CAT(a,b) a##b\nint main(void){return 25 CAT(,);}",
            25,
        ),
        (
            "stringify raw spelling",
            "#define STR(x) #x\nint main(void){return STR(hello)[1];}",
            101,
        ),
        (
            "stringify whitespace",
            "#define STR(x) #x\nint main(void){return sizeof(STR(a    b));}",
            4,
        ),
        (
            "stringify without prescan",
            "#define X 99\n#define STR(x) #x\nint main(void){return STR(X)[0];}",
            88,
        ),
        (
            "stringify quote escapes",
            "#define STR(x) #x\nint main(void){return STR(\"a\")[0];}",
            34,
        ),
        (
            "if expression",
            "#if (3*4 == 12) && !0\nint main(void){return 26;}\n#else\n#error wrong branch\n#endif",
            26,
        ),
        (
            "inactive diagnostics",
            "#if 0\n#error inactive\n#include <missing.h>\n#endif\nint main(void){return 27;}",
            27,
        ),
        (
            "elif selection",
            "#if 0\n#error first\n#elif 2>1\nint main(void){return 28;}\n#else\n#error last\n#endif",
            28,
        ),
        (
            "defined parentheses",
            "#define YES\n#if defined(YES) && !defined(NO)\nint main(void){return 29;}\n#endif",
            29,
        ),
        (
            "defined bare name",
            "#define YES 1\n#if defined YES\nint main(void){return 30;}\n#endif",
            30,
        ),
        (
            "ifdef and ifndef",
            "#define YES\n#ifdef YES\n#ifndef NO\nint main(void){return 31;}\n#endif\n#endif",
            31,
        ),
        (
            "unknown conditional identifier",
            "#if UNDEFINED\n#error bad\n#else\nint main(void){return 32;}\n#endif",
            32,
        ),
        (
            "conditional short circuit",
            "#if 1 || (1/0)\nint main(void){return 33;}\n#endif",
            33,
        ),
        (
            "conditional ternary",
            "#if 0 ? (1/0) : 1\nint main(void){return 34;}\n#endif",
            34,
        ),
        (
            "inactive nested if",
            "#if 0\n#if 1/0\n#error bad\n#endif\n#endif\nint main(void){return 35;}",
            35,
        ),
        (
            "line continuation",
            "#define VALUE 1 + \\\n 2\nint main(void){return VALUE;}",
            3,
        ),
        (
            "spliced identifier",
            "int ma\\\nin(void){return 36;}",
            36,
        ),
        (
            "trigraph directive",
            "??=define VALUE 37\nint main(void){return VALUE;}",
            37,
        ),
        (
            "trigraph brackets",
            "int main(void){int a??(2??);a??(0??)=38;return a??(0??);}",
            38,
        ),
        (
            "trigraph splice",
            "#define VALUE 3??/\n9\nint main(void){return VALUE;}",
            39,
        ),
        (
            "crlf input",
            "#define VALUE 40\r\nint main(void)\r\n{return VALUE;}\r\n",
            40,
        ),
        (
            "line comment splice",
            "int main(void){ // comment \\\n this is still a comment\nreturn 41;}",
            41,
        ),
        (
            "adjacent strings",
            "int main(void){return sizeof(\"ab\" \"cd\");}",
            5,
        ),
        (
            "escaped string",
            "int main(void){return \"a\\nb\"[1];}",
            10,
        ),
        (
            "apostrophe inside string",
            "int main(void){return \"a'b\"[1];}",
            39,
        ),
        (
            "octal character",
            "int main(void){return '\\101';}",
            65,
        ),
        (
            "hexadecimal character",
            "int main(void){return '\\x41';}",
            65,
        ),
        (
            "constant initializer",
            "int g=2+3*4;int main(void){return g;}",
            14,
        ),
    ];
    for (index, (purpose, source, expected)) in inputs.into_iter().enumerate() {
        cases.push(Case::new(
            format!("preprocess_{:03}", index + 1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
    let number = cases.len() + 1;
    cases.push(
        Case::new(
            format!("preprocess_{number:03}"),
            "quoted include and repeated guard",
            "#include \"local.h\"\n#include \"local.h\"\nint main(void){return INCLUDED;}",
            Expectation::Exit(43),
        )
        .with_files(vec![(
            "local.h",
            "#ifndef LOCAL_H\n#define LOCAL_H\n#define INCLUDED 43\n#endif\n",
        )]),
    );
    for (index, (source, purpose)) in [
        (
            "#if 1\nint main(void){return 0;}",
            "unterminated conditional",
        ),
        ("#else\nint main(void){return 0;}", "unmatched else"),
        ("#if 1\n#else\n#else\n#endif\n", "duplicate else"),
        (
            "#define F(a,b) a\nint main(void){return F(1);}",
            "too few macro arguments",
        ),
        (
            "#define F(a) a\nint main(void){return F(1,2);}",
            "too many macro arguments",
        ),
        (
            "#define CAT(a,b) a##b\nint main(void){return CAT(+,*);}",
            "invalid pasted token",
        ),
        (
            "#include <absent.h>\nint main(void){return 0;}",
            "missing embedded header",
        ),
        (
            "#error stop\nint main(void){return 0;}",
            "active error directive",
        ),
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(Case::new(
            format!("pp_errors_{:03}", index + 1),
            purpose,
            source,
            Expectation::Reject("E013".into()),
        ));
    }
    cases
}
