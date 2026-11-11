// Lifetime cases for aggregate call.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "union class merge",
            "union U{\
                double d;\
                long n;\
            };\
            union U f(union U x){\
                return x;\
            }\
            int main(void){\
                union U u;\
                u.n=7;\
                return f(u).n;\
            }",
            7,
        ),
        (
            "nested arrays and members",
            "struct S{\
                struct N{\
                    float a,b;\
                } \
                n;\
                int a[2];\
            };\
            struct S f(struct S x){\
                return x;\
            }\
            int main(void){\
                struct S s={{\
                    1,2},{\
                    3,1}};\
                s=f(s);\
                return (int)(s.n.a+s.n.b)+s.a[0]+s.a[1];\
            }",
            7,
        ),
        (
            "parameter modifications are private",
            "struct S{\
                long a,b,c;\
            };\
            long f(struct S s){\
                s.a=9;\
                return s.a;\
            }\
            int main(void){\
                struct S s={\
                    7,2,3};\
                f(s);\
                return s.a;\
            }",
            7,
        ),
        (
            "register parameter modifications are private",
            "struct S{\
                int a;\
            };\
            int f(struct S s){\
                s.a=9;\
                return s.a;\
            }\
            int main(void){\
                struct S s={7};\
                f(s);\
                return s.a;\
            }",
            7,
        ),
        (
            "nested calls preserve result objects",
            "struct S{\
                long a,b,c;\
            };\
            struct S f(int x){\
                struct S s={\
                    x,x,x};\
                return s;\
            }\
            long g(struct S a,struct S b){\
                return a.a+b.b;\
            }\
            int main(void){\
                return g(f(3),f(4));\
            }",
            7,
        ),
        (
            "recursive structure returns",
            "struct S{\
                long x,y,z;\
            };\
            struct S f(int n){\
                if(n)return f(n-1);\
                struct S s={\
                    1,2,4};\
                return s;\
            }\
            int main(void){\
                struct S s=f(5);\
                return s.x+s.y+s.z;\
            }",
            7,
        ),
        (
            "aggregate function pointer",
            "struct S{\
                int x;\
            };\
            struct S f(int x){\
                struct S s={x};\
                return s;\
            }\
            int main(void){\
                struct S (*p)(int)=f;\
                return p(7).x;\
            }",
            7,
        ),
        (
            "return result used as argument",
            "struct S{\
                double x;\
                int y;\
            };\
            struct S f(void){\
                struct S s={\
                    3,4};\
                return s;\
            }\
            int g(struct S s){\
                return (int)s.x+s.y;\
            }\
            int main(void){\
                return g(f());\
            }",
            7,
        ),
        (
            "conditional aggregate return",
            "struct S{\
                int x;\
            };\
            struct S f(int n){\
                struct S a={7},b={9};\
                return n?a:b;\
            }\
            int main(void){\
                return f(1).x;\
            }",
            7,
        ),
    ] {
        cases.push(Case::new(
            format!("aggregate_call_{:03}", cases.len() + 1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
}
