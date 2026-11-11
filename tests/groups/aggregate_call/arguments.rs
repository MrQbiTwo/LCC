// Arguments cases for aggregate call.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "one integer chunk",
            "struct S{\
                int a,b;\
            };\
            int f(struct S s){\
                return s.a+s.b;\
            }\
            int main(void){\
                struct S s={\
                    3,4};\
                return f(s);\
            }",
            7,
        ),
        (
            "two integer chunks",
            "struct S{\
                long a,b;\
            };\
            long f(struct S s){\
                return s.a+s.b;\
            }\
            int main(void){\
                struct S s={\
                    3,4};\
                return f(s);\
            }",
            7,
        ),
        (
            "two SSE chunks",
            "struct S{\
                double a,b;\
            };\
            double f(struct S s){\
                return s.a+s.b;\
            }\
            int main(void){\
                struct S s={\
                    3,4};\
                return (int)f(s);\
            }",
            7,
        ),
        (
            "integer then SSE",
            "struct S{\
                long a;\
                double b;\
            };\
            double f(struct S s){\
                return s.a+s.b;\
            }\
            int main(void){\
                struct S s={\
                    3,4};\
                return (int)f(s);\
            }",
            7,
        ),
        (
            "SSE then integer",
            "struct S{\
                double a;\
                long b;\
            };\
            double f(struct S s){\
                return s.a+s.b;\
            }\
            int main(void){\
                struct S s={\
                    3,4};\
                return (int)f(s);\
            }",
            7,
        ),
        (
            "packed floats share SSE chunk",
            "struct S{\
                float a,b,c;\
            };\
            float f(struct S s){\
                return s.a+s.b+s.c;\
            }\
            int main(void){\
                struct S s={\
                    1,2,4};\
                return (int)f(s);\
            }",
            7,
        ),
        (
            "large stack aggregate",
            "struct S{\
                long a,b,c;\
            };\
            long f(struct S s,int n){\
                return s.a+s.b+s.c+n;\
            }\
            int main(void){\
                struct S s={\
                    1,2,3};\
                return f(s,1);\
            }",
            7,
        ),
        (
            "binary80 aggregate argument",
            "struct S{\
                long double a;\
            };\
            long double f(struct S s){\
                return s.a;\
            }\
            int main(void){\
                struct S s={7};\
                return (int)f(s);\
            }",
            7,
        ),
        (
            "short parameter copy boundary",
            "struct S{\
                char a[3];\
            };\
            int f(int guard,struct S s){\
                return guard+s.a[0]+s.a[1]+s.a[2];\
            }\
            int main(void){\
                struct S s={{\
                    1,2,3}};\
                return f(1,s);\
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
