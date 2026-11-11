// Static lifetime cases for storage.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "zero-initialized local static persists",
            "int f(void){static int x;return ++x;}int main(void){int a=f();int b=f();return a*10+b;}",
            12,
        ),
        (
            "explicit local static initializes once",
            "int f(void){static int x=7;return x++;}int main(void){int a=f();int b=f();return a+b;}",
            15,
        ),
        (
            "recursive calls share local static",
            "int f(int n){static int x;x++;if(n)return f(n-1);return x;}int main(void){return f(3);}",
            4,
        ),
        (
            "local static survives unrelated stack writes",
            "int *f(void){\
                static int x=13;\
                return &x;\
            }\
            int g(void){\
                int a[20];\
                a[0]=3;\
                a[19]=4;\
                return a[0]+a[19];\
            }\
            int main(void){\
                int *p=f();\
                int n=g();\
                *p+=n;\
                return *f();\
            }",
            20,
        ),
        (
            "local static address is stable",
            "int *f(void){\
                static int x;\
                return &x;\
            }\
            int main(void){\
                int *p=f();\
                *p=17;\
                return p==f()&&*f()==17;\
            }",
            1,
        ),
        (
            "same names in functions have distinct storage",
            "int *f(void){\
                static int x=3;\
                return &x;\
            }\
            int *g(void){\
                static int x=5;\
                return &x;\
            }\
            int main(void){\
                int *p=f();\
                int *q=g();\
                *p=7;\
                return p!=q&&*q==5&&*f()==7;\
            }",
            1,
        ),
        (
            "nested static shadows and restores outer static",
            "int f(void){\
                static int x=3;\
                int n;\
                {\
                    static int x=7;\
                    n=x++;\
                }\
                return x+++n;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            22,
        ),
        (
            "sibling blocks use distinct static objects",
            "int main(void){\
                int *p;\
                int *q;\
                {\
                    static int x=3;\
                    p=&x;\
                }\
                {\
                    static int x=5;\
                    q=&x;\
                }\
                *p=9;\
                return p!=q&&*q==5;\
            }",
            1,
        ),
        (
            "automatic shadow does not alter static",
            "int f(void){\
                static int x=7;\
                {\
                    int x=13;\
                    x++;\
                }\
                return ++x;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            17,
        ),
        (
            "local static shadows file scope object",
            "int x=19;int f(void){static int x=3;return ++x;}int main(void){int n=f();return x+n;}",
            23,
        ),
        (
            "skipped declaration still has initialized storage",
            "int main(void){goto use;{static int x=17;use:return x;}}",
            17,
        ),
        (
            "conditional block retains static state",
            "int f(int n){\
                if(n){\
                    static int x=3;\
                    return x++;\
                }\
                return 0;\
            }\
            int main(void){\
                int a=f(1);\
                int b=f(0);\
                int c=f(1);\
                return a+b+c;\
            }",
            7,
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
