// Enumeration values use checked target integers and a separate scoped tag namespace.
use crate::{Case, Expectation};

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (purpose, source, expected) in [
        (
            "implicit sequence",
            "enum color {red,green,blue};int main(void){return red+10*green+100*blue;}",
            210,
        ),
        (
            "explicit reset",
            "enum {a=3,b,c=9,d};int main(void){return a+b+c+d;}",
            26,
        ),
        (
            "previous constant reference",
            "enum {a=3,b=a*2,c=b+1};int main(void){return c;}",
            7,
        ),
        (
            "negative enumerator",
            "enum {a=-3,b,c};int main(void){return a+b+c;}",
            250,
        ),
        (
            "tagged object",
            "enum color {red=2,green=7};enum color c=green;int main(void){return c;}",
            7,
        ),
        (
            "definition with object",
            "enum color {red=2,green=7} c=green;int main(void){return c;}",
            7,
        ),
        (
            "typedef enum",
            "typedef enum {red=2,green=7} color;int main(void){color c=green;return c;}",
            7,
        ),
        (
            "local enumeration",
            "int main(void){enum {a=3,b=a+4};return b;}",
            7,
        ),
        (
            "constant array dimension",
            "enum {count=7};int main(void){int a[count];return sizeof(a);}",
            28,
        ),
        (
            "case constant",
            "enum {value=7};int main(void){switch(7){case value:return 9;}return 0;}",
            9,
        ),
        (
            "tag and constant namespaces",
            "enum color {color=7};int main(void){enum color c=color;return c;}",
            7,
        ),
        (
            "inner enumerator shadows outer",
            "enum {a=1};int main(void){int x=a;{enum {a=6};x+=a;}return x;}",
            7,
        ),
        (
            "inner tag shadows outer",
            "enum E {a=1};int main(void){enum E x=a;{enum E {b=6};enum E y=b;x+=y;}return x;}",
            7,
        ),
        (
            "tag scope restored",
            "enum E {a=7};int main(void){{enum E {b=3};}enum E x=a;return x;}",
            7,
        ),
        (
            "function parameter enumeration",
            "enum E {a=7};int f(enum E e){return e;}int main(void){return f(a);}",
            7,
        ),
        (
            "target enum width",
            "enum E {a=7};int main(void){return sizeof(enum E);}",
            4,
        ),
        (
            "trailing comma extension",
            "enum E {a=7,};int main(void){return a;}",
            7,
        ),
        (
            "largest signed enumerator",
            "enum {a=2147483647};int main(void){return a>0;}",
            1,
        ),
        (
            "smallest signed enumerator",
            "enum {a=-2147483647-1};int main(void){return a<0;}",
            1,
        ),
    ] {
        cases.push(Case::new(
            format!("enum_{:03}", cases.len() + 1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
    for (source, code) in [
        ("enum {a,a};int main(void){return 0;}", "E009"),
        ("enum E {a};enum E {b};int main(void){return 0;}", "E009"),
        ("enum {a};int a;int main(void){return 0;}", "E009"),
        ("int a;enum {a};int main(void){return 0;}", "E009"),
        ("enum {a=2147483648L};int main(void){return 0;}", "E011"),
        ("enum {a=2147483647,b};int main(void){return 0;}", "E011"),
        ("enum {a=1.5};int main(void){return 0;}", "E008"),
        ("enum {a=1/0};int main(void){return 0;}", "E011"),
        ("enum E x;int main(void){return 0;}", "E008"),
        ("enum {a=1};int main(void){a=2;return 0;}", "E008"),
        ("int main(void){{enum {a=1};}return a;}", "E009"),
    ] {
        cases.push(Case::new(
            format!("enum_{:03}", cases.len() + 1),
            source,
            source,
            Expectation::Reject(code.into()),
        ));
    }
    cases
}
