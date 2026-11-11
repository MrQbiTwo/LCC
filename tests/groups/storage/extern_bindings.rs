// Extern bindings cases for storage.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "block extern binds visible global",
            "int x=13;int f(void){extern int x;return ++x;}int main(void){int n=f();return n+x;}",
            28,
        ),
        (
            "block extern inherits visible internal linkage",
            "static int x=17;int f(void){extern int x;return ++x;}int main(void){int n=f();return n+x;}",
            36,
        ),
        (
            "block extern hides automatic then restores it",
            "int x=7;int main(void){int x=3;int n;{extern int x;x=11;n=x;}return n+x;}",
            14,
        ),
        (
            "forward block extern object",
            "int f(void){extern int x;return ++x;}int x=7;int main(void){int n=f();return n+x;}",
            16,
        ),
        (
            "sibling block extern declarations share identity",
            "int main(void){\
                int *p;\
                int *q;\
                {\
                    extern int x;\
                    p=&x;\
                }\
                {\
                    extern int x;\
                    q=&x;\
                }\
                *p=9;\
                return p==q&&*q==9;\
            }\
            int x;",
            1,
        ),
        (
            "repeated block extern object declarations",
            "int x=13;int main(void){extern int x;extern int x;return x;}",
            13,
        ),
        (
            "nested compatible block extern declarations",
            "int x=17;int main(void){extern int x;{extern int x;x++;}return x;}",
            18,
        ),
        (
            "block extern declarations across functions",
            "int f(void){\
                extern int x;\
                return ++x;\
            }\
            int g(void){\
                extern int x;\
                return ++x;\
            }\
            int x=3;\
            int main(void){\
                int a=f();\
                int b=g();\
                return a+b;\
            }",
            9,
        ),
        (
            "block extern retains visible complete array bound",
            "int a[3]={3,5,7};int main(void){extern int a[];return sizeof(a)+a[2];}",
            19,
        ),
        (
            "block extern supplies complete array bound",
            "extern int a[];\
            int f(void){\
                extern int a[3];\
                return sizeof(a)+a[2];\
            }\
            int a[3]={\
                3,5,7};\
            int main(void){\
                return f();\
            }",
            19,
        ),
        (
            "same-scope extern completes earlier block bound",
            "int f(void){\
                extern int a[];\
                extern int a[3];\
                return sizeof(a)+a[2];\
            }\
            int a[3]={\
                3,5,7};\
            int main(void){\
                return f();\
            }",
            19,
        ),
        (
            "nested extern array reads shared storage",
            "int a[3]={\
                3,5,7};\
            int main(void){\
                extern int a[];\
                {\
                    extern int a[3];\
                    a[1]=11;\
                }\
                return sizeof(a)+a[1];\
            }",
            23,
        ),
        (
            "block extern incomplete array can remain unused",
            "int main(void){extern int missing[];return 7;}",
            7,
        ),
        (
            "block extern scalar can remain unused",
            "int main(void){extern int missing;return 7;}",
            7,
        ),
        (
            "block function declaration binds later definition",
            "int main(void){int f(int);return f(3);}int f(int x){return x+7;}",
            10,
        ),
        (
            "explicit block extern function binds later definition",
            "int main(void){extern int f(int);return f(4);}int f(int x){return x+7;}",
            11,
        ),
        (
            "block function inherits visible internal linkage",
            "static int f(int x){return x+7;}int main(void){int f(int);return f(5);}",
            12,
        ),
        (
            "repeated block function prototypes",
            "int main(void){int f(int);int f(int);return f(6);}int f(int x){return x+7;}",
            13,
        ),
        (
            "block function hides outer automatic name",
            "int main(void){int f=3;int n;{int f(int);n=f(7);}return n+f;}int f(int x){return x+7;}",
            17,
        ),
        (
            "local static uses block function address",
            "int main(void){int f(int);static int (*p)(int)=f;return p(8);}int f(int x){return x+7;}",
            15,
        ),
        (
            "block function declaration does not reserve local bytes",
            "int main(void){\
                int f(int);\
                char a[3];\
                a[0]=3;\
                a[1]=5;\
                a[2]=7;\
                return f(a[0]+a[1]+a[2]);\
            }\
            int f(int x){\
                return x;\
            }",
            15,
        ),
    ] {
        cases.push(Case::new(
            format!("storage_{:03}", cases.len()+1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
}
