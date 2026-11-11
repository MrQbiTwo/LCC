// Initializer trees cover data emission and local evaluation with the same cases.
use crate::{Case, Expectation};

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (purpose, declaration, expression, expected) in [
        ("scalar braces", "int a={7};", "a", 7),
        ("scalar trailing comma", "int a={7,};", "a", 7),
        ("integer array", "int a[3]={2,3,4};", "a[0]+a[1]+a[2]", 9),
        (
            "partial array zero fill",
            "int a[5]={7};",
            "a[0]+a[1]+a[4]",
            7,
        ),
        ("array trailing comma", "int a[2]={3,4,};", "a[0]+a[1]", 7),
        (
            "multidimensional array",
            "int a[2][3]={{1,2},{3,4,5}};",
            "a[0][0]+a[0][1]+a[0][2]+a[1][2]",
            8,
        ),
        (
            "elided array braces",
            "int a[2][3]={1,2,3,4,5,6};",
            "a[1][0]+a[1][2]",
            10,
        ),
        (
            "partial elided inner array",
            "int a[2][3]={1,2,3,4};",
            "a[1][0]+a[1][1]+a[1][2]",
            4,
        ),
        (
            "mixed array braces",
            "int a[2][2]={{1},2,3};",
            "a[0][1]+a[1][0]+a[1][1]",
            5,
        ),
        (
            "structure padding",
            "struct S{char c;long x;short y;} a={1,2,4};",
            "a.c+a.x+a.y",
            7,
        ),
        (
            "partial structure zero fill",
            "struct S{int x;int y;} a={7};",
            "a.x+a.y",
            7,
        ),
        (
            "nested structure",
            "struct S{int x;struct T{char c;long y;} t;} a={1,{2,4}};",
            "a.x+a.t.c+a.t.y",
            7,
        ),
        (
            "elided nested structure",
            "struct S{int x;struct T{char c;long y;} t;} a={1,2,4};",
            "a.x+a.t.c+a.t.y",
            7,
        ),
        (
            "array of structures",
            "struct S{int x,y;} a[2]={{1,2},{3,4}};",
            "a[0].x+a[1].x+a[1].y",
            8,
        ),
        (
            "elided array of structures",
            "struct S{int x,y;} a[2]={1,2,3,4};",
            "a[0].y+a[1].x+a[1].y",
            9,
        ),
        (
            "array member then scalar",
            "struct S{int x[3];int y;} a={{1,2},4};",
            "a.x[0]+a.x[1]+a.x[2]+a.y",
            7,
        ),
        (
            "elided member array",
            "struct S{int x[2];int y;} a={1,2,4};",
            "a.x[0]+a.x[1]+a.y",
            7,
        ),
        (
            "union first member",
            "union U{int x;double y;} a={7};",
            "a.x",
            7,
        ),
        (
            "union aggregate first member",
            "union U{struct S{int x,y;} s;long q;} a={{3,4}};",
            "a.s.x+a.s.y",
            7,
        ),
        (
            "union elided first member",
            "union U{struct S{int x,y;} s;long q;} a={3,4};",
            "a.s.x+a.s.y",
            7,
        ),
        (
            "character array string",
            "char a[4]=\"abc\";",
            "a[0]+a[1]+a[2]+a[3]-287",
            7,
        ),
        ("string padding", "char a[8]=\"abc\";", "a[3]+a[7]", 0),
        (
            "string omits terminator at bound",
            "char a[3]=\"abc\";",
            "a[2]",
            99,
        ),
        ("braced string", "char a[4]={\"abc\",};", "a[0]+a[3]", 97),
        ("empty string", "char a[2]=\"\";", "a[0]+a[1]", 0),
        (
            "embedded null string",
            "char a[5]=\"a\\0b\";",
            "a[1]+a[2]+a[4]",
            98,
        ),
        (
            "adjacent string literals",
            "char a[5]=\"ab\" \"cd\";",
            "a[0]+a[3]-190",
            7,
        ),
        (
            "string member",
            "struct S{char x[4];int y;} a={\"abc\",7};",
            "a.x[3]+a.y",
            7,
        ),
        (
            "array of strings",
            "char a[2][4]={\"ab\",\"cd\"};",
            "a[0][2]+a[1][1]",
            100,
        ),
        (
            "floating members",
            "struct S{float f;double d;long double l;} a={1.5f,2.25,3.25L};",
            "(int)(a.f+a.d+a.l)",
            7,
        ),
        (
            "floating array conversion",
            "double a[3]={1,2.5f,3.5L};",
            "(int)(a[0]+a[1]+a[2])",
            7,
        ),
        (
            "extended padding and next object",
            "struct S{long double l;char c;} a={6.0L,1};int b=99;",
            "(int)a.l+a.c+(b!=99)",
            7,
        ),
        (
            "const aggregate initializer",
            "const struct S{int x,y;} a={3,4};",
            "a.x+a.y",
            7,
        ),
        (
            "const member initializer",
            "struct S{const int x;int y;} a={3,4};",
            "a.x+a.y",
            7,
        ),
        (
            "unsigned member conversion",
            "struct S{unsigned char x;unsigned short y;} a={257,65542};",
            "a.x+a.y",
            7,
        ),
        (
            "enum constant initializer",
            "enum E{N=3};struct S{int a[N];} a={{1,N,N}};",
            "a.a[0]+a.a[1]+a.a[2]",
            7,
        ),
    ] {
        for local in [false, true] {
            let source = if local {
                format!("int main(void){{{declaration}return {expression};}}")
            } else {
                format!("{declaration}int main(void){{return {expression};}}")
            };
            cases.push(Case::new(
                format!("initializer_{:03}", cases.len() + 1),
                format!("{} {purpose}", if local { "local" } else { "global" }),
                source,
                Expectation::Exit(expected),
            ));
        }
    }
    for (purpose, source, expected) in [
        (
            "local function call elements",
            "int n;\
            int f(void){\
                n++;\
                return n;\
            }\
            int main(void){\
                int a[3]={\
                    f(),f(),f()};\
                return a[0]+a[1]+a[2]+n;\
            }",
            9,
        ),
        (
            "local aggregate member copy",
            "struct S{\
                int x,y;\
            };\
            struct T{\
                struct S s;\
                int z;\
            };\
            int main(void){\
                struct S s={\
                    1,2};\
                struct T t={\
                    s,4};\
                return t.s.x+t.s.y+t.z;\
            }",
            7,
        ),
        (
            "local pointer member initializer",
            "struct S{int *p;};int main(void){int n=7;struct S s={&n};return *s.p;}",
            7,
        ),
        (
            "local array of pointers",
            "int main(void){int x=3,y=4;int *a[2]={&x,&y};return *a[0]+*a[1];}",
            7,
        ),
        (
            "array declared beside scalar",
            "int main(void){int a[2]={3,4},b=7;return a[0]+a[1]+b;}",
            14,
        ),
        (
            "brace nested scalar extension",
            "int main(void){int a={{7}};return a;}",
            7,
        ),
    ] {
        cases.push(Case::new(
            format!("initializer_{:03}", cases.len() + 1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
    for (purpose, source, code) in [
        (
            "too many array elements",
            "int a[2]={1,2,3};int main(void){return 0;}",
            "E006",
        ),
        (
            "too many structure members",
            "struct S{int x;} a={1,2};int main(void){return 0;}",
            "E006",
        ),
        (
            "multiple union members",
            "union U{int x,y;} a={1,2};int main(void){return 0;}",
            "E006",
        ),
        (
            "too many scalar elements",
            "int a={1,2};int main(void){return 0;}",
            "E006",
        ),
        (
            "string exceeds array",
            "char a[2]=\"abc\";int main(void){return 0;}",
            "E008",
        ),
        (
            "array requires braces",
            "int a[2]=1;int main(void){return 0;}",
            "E008",
        ),
        (
            "structure scalar initializer",
            "struct S{int x;} a=7;int main(void){return 0;}",
            "E008",
        ),
        (
            "nonconstant global element",
            "int f(void){return 7;}int a[2]={f(),1};int main(void){return 0;}",
            "E008",
        ),
        (
            "empty initializer not C89",
            "int a[2]={};int main(void){return 0;}",
            "E006",
        ),
        (
            "whole global object is not constant",
            "struct S{int x;} a={7},b=a;int main(void){return 0;}",
            "E008",
        ),
        (
            "member of assignment is not modifiable",
            "struct S{int x;};int main(void){struct S a={1},b={2};(a=b).x=7;return 0;}",
            "E008",
        ),
        (
            "member of conditional is not modifiable",
            "struct S{int x;};int main(void){struct S a={1},b={2};(1?a:b).x=7;return 0;}",
            "E008",
        ),
        (
            "member of comma has no address",
            "struct S{int x;};int main(void){struct S a={1};&(1,a).x;return 0;}",
            "E008",
        ),
        (
            "cast to aggregate",
            "struct S{int x;};int main(void){struct S a={1};(struct S)a;return 0;}",
            "E008",
        ),
        (
            "aggregate cast to scalar",
            "struct S{int x;};int main(void){struct S a={1};return (long)a;}",
            "E008",
        ),
    ] {
        cases.push(Case::new(
            format!("initializer_{:03}", cases.len() + 1),
            purpose,
            source,
            Expectation::Reject(code.into()),
        ));
    }
    cases
}
