// Inferred objects cases for array bounds.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, declaration, expression, expected) in [
        ("integer bound", "int a[]={2,3,4};", "sizeof(a)+a[2]", 16),
        ("trailing comma", "int a[]={7,};", "sizeof(a)+a[0]", 11),
        (
            "floating width",
            "double a[]={1.5,2.5};",
            "sizeof(a)+(int)(a[0]+a[1])",
            20,
        ),
        (
            "binary80 alignment",
            "long double a[]={1.0L,2.0L};int b=7;",
            "sizeof(a)+(int)a[1]+b",
            41,
        ),
        (
            "narrow conversion",
            "unsigned char a[]={257,258};",
            "sizeof(a)+a[0]+a[1]",
            5,
        ),
        (
            "string terminator",
            "char a[]=\"abc\";",
            "sizeof(a)+a[3]",
            4,
        ),
        ("empty string", "char a[]=\"\";", "sizeof(a)+a[0]", 1),
        (
            "concatenated string",
            "char a[]=\"ab\" \"cd\";",
            "sizeof(a)+a[4]",
            5,
        ),
        (
            "embedded null string",
            "char a[]=\"a\\0b\";",
            "sizeof(a)+a[1]+a[2]",
            102,
        ),
        ("braced string", "char a[]={\"abc\",};", "sizeof(a)+a[3]", 4),
        (
            "const elements",
            "const int a[]={3,4};",
            "sizeof(a)+a[0]+a[1]",
            15,
        ),
        (
            "nested partial arrays",
            "int a[][3]={{1,2},{3}};",
            "sizeof(a)+a[0][2]+a[1][0]+a[1][2]",
            27,
        ),
        (
            "elided partial arrays",
            "int a[][3]={1,2,3,4};",
            "sizeof(a)+a[1][0]+a[1][2]",
            28,
        ),
        (
            "mixed nested braces",
            "int a[][2]={{1},2,3};",
            "sizeof(a)+a[0][1]+a[1][1]",
            19,
        ),
        (
            "array of strings",
            "char a[][4]={\"a\",\"bc\"};",
            "sizeof(a)+a[0][3]+a[1][2]",
            8,
        ),
        (
            "record array",
            "struct S{int x,y;} a[]={{1},{2,3}};",
            "sizeof(a)+a[0].y+a[1].y",
            19,
        ),
        (
            "elided record array",
            "struct S{int x,y;} a[]={1,2,3};",
            "sizeof(a)+a[1].x+a[1].y",
            19,
        ),
        (
            "bit-field records",
            "struct S{unsigned x:3;int y:5;} a[]={{7,-2},{3,4}};",
            "sizeof(a)+a[0].y+a[1].x",
            9,
        ),
        (
            "union array",
            "union U{int x;double d;} a[]={{3},{4}};",
            "sizeof(a)+a[0].x+a[1].x",
            23,
        ),
        (
            "independent typedef bounds",
            "typedef int A[];A a={1},b={2,3};",
            "sizeof(a)+sizeof(b)+b[1]",
            15,
        ),
        (
            "qualified typedef bounds",
            "typedef int A[];const A a={1},b={2,3,4};",
            "sizeof(a)+sizeof(b)+b[2]",
            20,
        ),
        (
            "typedef element array",
            "typedef int Row[2];Row a[]={{1},{2,3}};",
            "sizeof(a)+a[1][1]",
            19,
        ),
        (
            "multiple inferred declarators",
            "int a[]={1},b[]={2,3},c=4;",
            "sizeof(a)+sizeof(b)+c",
            16,
        ),
    ] {
        for local in [false, true] {
            let source = if local {
                format!("int main(void){{{declaration}return {expression};}}")
            } else {
                format!("{declaration}int main(void){{return {expression};}}")
            };
            cases.push(Case::new(
                format!("array_bounds_{:03}", cases.len() + 1),
                format!("{} {purpose}", if local { "local" } else { "global" }),
                source,
                Expectation::Exit(expected),
            ));
        }
    }
}
