// Aggregate allocation cases for vararg.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, source) in [
        (
            "named_mixed",
            "struct S{\
                double x;\
                long y;\
            };\
            int f(struct S n,...){\
                va_list a;\
                double x;\
                long y;\
                va_start(a,n);\
                x=va_arg(a,double);\
                y=va_arg(a,long);\
                return n.x==1&&n.y==2&&x==3&&y==4;\
            }\
            int main(void){\
                struct S s={\
                    1,2};\
                return f(s,3.0,4L);\
            }",
        ),
        (
            "named_memory",
            "struct S{\
                long x,y,z;\
            };\
            int f(struct S n,...){\
                va_list a;\
                long double x;\
                va_start(a,n);\
                x=va_arg(a,long double);\
                return n.x==1&&n.y==2&&n.z==3&&x==4;\
            }\
            int main(void){\
                struct S s={\
                    1,2,3};\
                return f(s,4.0L);\
            }",
        ),
        (
            "hidden_result",
            "struct S{\
                long x,y,z;\
            };\
            struct S f(int n,...){\
                va_list a;\
                struct S s;\
                int i;\
                va_start(a,n);\
                s.x=0;\
                for(i=0;i<7;i++)s.x+=va_arg(a,int);\
                s.y=a[0].gp_offset;\
                s.z=3;\
                return s;\
            }\
            int main(void){\
                struct S s=f(7,1,2,3,4,5,6,7);\
                return s.x==28&&s.y==48&&s.z==3;\
            }",
        ),
        (
            "gp_rollback",
            "struct S{\
                long x,y;\
            };\
            int f(int a,int b,int c,int d,int n,...){\
                va_list v;\
                struct S s;\
                long x;\
                va_start(v,n);\
                s=va_arg(v,struct S);\
                x=va_arg(v,long);\
                return s.x==6&&s.y==7&&x==8;\
            }\
            int main(void){\
                struct S s={\
                    6,7};\
                return f(1,2,3,4,5,s,8L);\
            }",
        ),
        (
            "sse_rollback",
            "struct S{\
                double x,y;\
            };\
            int f(double a,double b,double c,double d,double e,double f,double n,...){\
                va_list v;\
                struct S s;\
                double x;\
                va_start(v,n);\
                s=va_arg(v,struct S);\
                x=va_arg(v,double);\
                return s.x==6&&s.y==7&&x==8;\
            }\
            int main(void){\
                struct S s={\
                    6,7};\
                return f(1.,2.,3.,4.,5.,6.,7.,s,8.);\
            }",
        ),
        (
            "mixed_gp_rollback",
            "struct S{\
                double x;\
                long y;\
            };\
            int f(int a,int b,int c,int d,int e,int n,...){\
                va_list v;\
                struct S s;\
                double x;\
                va_start(v,n);\
                s=va_arg(v,struct S);\
                x=va_arg(v,double);\
                return s.x==6&&s.y==7&&x==8;\
            }\
            int main(void){\
                struct S s={\
                    6,7};\
                return f(1,2,3,4,5,6,s,8.);\
            }",
        ),
        (
            "mixed_sse_rollback",
            "struct S{\
                long x;\
                double y;\
            };\
            int f(double a,double b,double c,double d,double e,double f,double g,double n,...){\
                va_list v;\
                struct S s;\
                long x;\
                va_start(v,n);\
                s=va_arg(v,struct S);\
                x=va_arg(v,long);\
                return s.x==6&&s.y==7&&x==8;\
            }\
            int main(void){\
                struct S s={\
                    6,7};\
                return f(1.,2.,3.,4.,5.,6.,7.,8.,s,8L);\
            }",
        ),
    ] {
        cases.push(Case::new(
            format!("vararg_{name}"),
            name,
            format!("#include <stdarg.h>\n{source}"),
            Expectation::Exit(1),
        ));
    }
}
