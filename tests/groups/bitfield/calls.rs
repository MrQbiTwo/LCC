// Calls cases for bitfield.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, purpose, source) in [
        (
            "call_return",
            "bit-field values survive aggregate argument and result paths",
            "struct S{\
                unsigned a:3;\
                signed int b:5;\
            };\
            struct S f(struct S x){\
                x.a+=3;\
                x.b-=2;\
                return x;\
            }\
            int main(void){\
                struct S s={\
                    6,-7};\
                struct S t=f(s);\
                return s.a==6&&s.b==-7&&t.a==1&&t.b==-9&&f(t).a==4;\
            }",
        ),
        (
            "mixed_call",
            "mixed SSE and INTEGER aggregate classes include bit-field units",
            "struct S{\
                double d;\
                unsigned a:5,b:27;\
            };\
            struct S f(struct S s){\
                s.a++;\
                s.d+=1;\
                return s;\
            }\
            int main(void){\
                struct S s={\
                    2.5,17,0x123456};\
                s=f(s);\
                return s.d==3.5&&s.a==18&&s.b==0x123456;\
            }",
        ),
        (
            "vararg_promotion",
            "variadic scalar arguments apply field-width integer promotions",
            "#include <stdarg.h>\n\
            struct S{\
                unsigned a:4,b:32;\
            };\
            int f(int n,...){\
                va_list v;\
                int a;\
                unsigned b;\
                va_start(v,n);\
                a=va_arg(v,int);\
                b=va_arg(v,unsigned int);\
                return a==7&&b==4294967295U;\
            }\
            int main(void){\
                struct S s={\
                    7,4294967295U};\
                return f(0,s.a,s.b);\
            }",
        ),
        (
            "vararg_aggregate",
            "variadic aggregate fetch reconstructs packed field storage",
            "#include <stdarg.h>\n\
            struct S{\
                double d;\
                unsigned a:5,b:27;\
            };\
            int f(int n,...){\
                va_list v;\
                struct S s;\
                int tail;\
                va_start(v,n);\
                s=va_arg(v,struct S);\
                tail=va_arg(v,int);\
                return s.d==2.5&&s.a==17&&s.b==0x123456&&tail==9;\
            }\
            int main(void){\
                struct S s={\
                    2.5,17,0x123456};\
                return f(0,s,9);\
            }",
        ),
        (
            "vararg_overflow",
            "bit-field aggregates use overflow memory when GP registers exhaust",
            "#include <stdarg.h>\n\
            struct S{\
                unsigned a:3;\
                signed int b:5;\
            };\
            int f(int a,int b,int c,int d,int e,int n,...){\
                va_list v;\
                struct S s;\
                int tail;\
                va_start(v,n);\
                s=va_arg(v,struct S);\
                tail=va_arg(v,int);\
                return s.a==5&&s.b==-7&&tail==9;\
            }\
            int main(void){\
                struct S s={\
                    5,-7};\
                return f(1,2,3,4,5,6,s,9);\
            }",
        ),
    ] {
        cases.push(Case::new(
            format!("bitfield_{name}"),
            purpose,
            source,
            Expectation::Exit(1),
        ));
    }
}
