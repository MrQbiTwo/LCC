// Static values cases for storage.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "local static integer array inference and persistence",
            "int f(void){\
                static int a[]={\
                    3,5,7};\
                a[1]++;\
                return sizeof(a)+a[1];\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            37,
        ),
        (
            "local static zero array",
            "int f(void){\
                static int a[3];\
                a[1]++;\
                return a[0]+a[1]+a[2];\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            3,
        ),
        (
            "local static inferred string is writable",
            "int f(void){\
                static char s[]=\"abc\";\
                s[0]++;\
                return s[0]-'a'+sizeof(s);\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            11,
        ),
        (
            "local static nested array layout",
            "int f(void){\
                static int a[][2]={{\
                    1,2},{\
                    3,4}};\
                a[1][0]++;\
                return a[1][0]+sizeof(a);\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            41,
        ),
        (
            "unknown-bound typedef statics complete independently",
            "typedef int A[];\
            int main(void){\
                static A a={\
                    1,2};\
                static A b={\
                    3,4,5};\
                return sizeof(a)+sizeof(b)+a[1]+b[2];\
            }",
            27,
        ),
        (
            "local static padded record",
            "struct S{\
                char c;\
                long n;\
            };\
            int f(void){\
                static struct S s={\
                    3,11};\
                s.n++;\
                return s.c+s.n;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            31,
        ),
        (
            "local static record zero fill",
            "struct S{\
                int a;\
                int b[3];\
            };\
            int main(void){\
                static struct S s={\
                    7,{3}};\
                return s.a+s.b[0]+s.b[1]+s.b[2];\
            }",
            10,
        ),
        (
            "local static union",
            "union U{\
                long x;\
                char bytes[8];\
            };\
            int f(void){\
                static union U u={19};\
                return ++u.x;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            41,
        ),
        (
            "local static bit fields",
            "struct S{\
                unsigned a:3;\
                signed int b:5;\
            };\
            int f(void){\
                static struct S s={\
                    5,-3};\
                s.b++;\
                return s.a+s.b;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            7,
        ),
        (
            "local static floating values",
            "int f(void){\
                static float a=1.5f;\
                static double b=2.25;\
                static long double c=3.125L;\
                a+=1.0f;\
                b+=0.5;\
                c+=0.25L;\
                return (int)(a+b+c);\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            18,
        ),
        (
            "local static pointer to earlier static",
            "int f(void){\
                static int x=7;\
                static int *p=&x;\
                return ++*p;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            17,
        ),
        (
            "local static pointer to external object",
            "int x=13;int f(void){static int *p=&x;return ++*p;}int main(void){int n=f();return n+x;}",
            28,
        ),
        (
            "local static pointer into static array",
            "int f(void){\
                static int a[]={\
                    3,5,7};\
                static int *p=a+1;\
                return ++*p;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            13,
        ),
        (
            "local static pointer into string",
            "int main(void){static char *p=\"abc\"+1;return p[0];}",
            98,
        ),
        (
            "local static pointer to own object",
            "int main(void){static void *p=&p;return p==&p;}",
            1,
        ),
        (
            "local static self-referential record",
            "struct N{\
                int n;\
                struct N *next;\
            };\
            int main(void){\
                static struct N n={\
                    23,&n};\
                return n.next==&n&&n.next->n==23;\
            }",
            1,
        ),
        (
            "local static function pointer",
            "int f(int x){return x+7;}int main(void){static int (*p)(int)=f;return p(3);}",
            10,
        ),
        (
            "local static pointer table",
            "int main(void){static int a=3,b=5;static int *p[]={&a,&b};*p[1]+=*p[0];return b;}",
            8,
        ),
        (
            "local static const and volatile objects",
            "int f(void){\
                static const int n=7;\
                static volatile int x=3;\
                x+=n;\
                return x;\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return a+b;\
            }",
            27,
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
