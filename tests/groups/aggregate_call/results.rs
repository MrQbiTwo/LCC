// Results cases for aggregate call.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "integer result",
            "struct S{\
                long a,b;\
            };\
            struct S f(void){\
                struct S s={\
                    3,4};\
                return s;\
            }\
            int main(void){\
                struct S s=f();\
                return s.a+s.b;\
            }",
            7,
        ),
        (
            "SSE result",
            "struct S{\
                double a,b;\
            };\
            struct S f(void){\
                struct S s={\
                    3,4};\
                return s;\
            }\
            int main(void){\
                struct S s=f();\
                return (int)(s.a+s.b);\
            }",
            7,
        ),
        (
            "mixed result",
            "struct S{\
                long a;\
                double b;\
            };\
            struct S f(void){\
                struct S s={\
                    3,4};\
                return s;\
            }\
            int main(void){\
                struct S s=f();\
                return s.a+(int)s.b;\
            }",
            7,
        ),
        (
            "reverse mixed result",
            "struct S{\
                double a;\
                long b;\
            };\
            struct S f(void){\
                struct S s={\
                    3,4};\
                return s;\
            }\
            int main(void){\
                return (int)f().a+f().b;\
            }",
            7,
        ),
        (
            "hidden result pointer",
            "struct S{\
                long a,b,c;\
            };\
            struct S f(int n){\
                struct S s={\
                    1,2,n};\
                return s;\
            }\
            int main(void){\
                struct S s=f(4);\
                return s.a+s.b+s.c;\
            }",
            7,
        ),
        (
            "extended aggregate return",
            "struct S{\
                long double a;\
            };\
            struct S f(long double x){\
                struct S s={x};\
                return s;\
            }\
            int main(void){\
                return (int)f(7.0L).a;\
            }",
            7,
        ),
        (
            "short result copy boundary",
            "struct S{\
                char a[3];\
            };\
            struct S f(void){\
                struct S s={{\
                    1,2,4}};\
                return s;\
            }\
            int main(void){\
                int g=99;\
                struct S s=f();\
                return s.a[0]+s.a[1]+s.a[2]+(g!=99);\
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
