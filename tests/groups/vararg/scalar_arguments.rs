// Scalar arguments cases for vararg.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, source) in [
        (
            "integers",
            "int f(int n,...){\
                va_list a;\
                int x,y;\
                va_start(a,n);\
                x=va_arg(a,int);\
                y=va_arg(a,int);\
                va_end(a);\
                return x+y;\
            }\
            int main(void){\
                return f(2,3,4)==7;\
            }",
        ),
        (
            "promotions",
            "int f(int n,...){\
                va_list a;\
                int x;\
                double y;\
                va_start(a,n);\
                x=va_arg(a,int);\
                y=va_arg(a,double);\
                return x==-3&&y==4.5;\
            }\
            int main(void){\
                signed char x=-3;\
                float y=4.5f;\
                return f(2,x,y);\
            }",
        ),
        (
            "unsigned",
            "int f(int n,...){\
                va_list a;\
                unsigned int x;\
                unsigned long y;\
                va_start(a,n);\
                x=va_arg(a,unsigned int);\
                y=va_arg(a,unsigned long);\
                return x==4294967295U&&y==18446744073709551615UL;\
            }\
            int main(void){\
                return f(2,4294967295U,18446744073709551615UL);\
            }",
        ),
        (
            "pointer",
            "int f(int n,...){\
                va_list a;\
                int *p;\
                va_start(a,n);\
                p=va_arg(a,int *);\
                return *p==7;\
            }\
            int main(void){\
                int x=7;\
                return f(1,&x);\
            }",
        ),
        (
            "function_pointer",
            "int g(int x){\
                return x+1;\
            }\
            int f(int n,...){\
                va_list a;\
                int (*p)(int);\
                va_start(a,n);\
                p=va_arg(a,int (*)(int));\
                return p(6)==7;\
            }\
            int main(void){\
                return f(1,g);\
            }",
        ),
        (
            "gp_overflow",
            "int f(int n,...){\
                va_list a;\
                int i,s=0;\
                va_start(a,n);\
                for(i=0;i<n;i++)s+=va_arg(a,int);\
                return s==45;\
            }\
            int main(void){\
                return f(9,1,2,3,4,5,6,7,8,9);\
            }",
        ),
        (
            "sse_overflow",
            "int f(double n,...){\
                va_list a;\
                int i;\
                double s=0;\
                va_start(a,n);\
                for(i=0;i<10;i++)s+=va_arg(a,double);\
                return s==55;\
            }\
            int main(void){\
                return f(0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0);\
            }",
        ),
        (
            "interleaved",
            "int f(int n,...){\
                va_list a;\
                int i,x,s=0;\
                double y;\
                va_start(a,n);\
                for(i=0;i<9;i++){\
                    x=va_arg(a,int);\
                    y=va_arg(a,double);\
                    s+=x+(int)y;\
                }\
                return s==90;\
            }\
            int main(void){\
                return f(9,1,1.0,2,2.0,3,3.0,4,4.0,5,5.0,6,6.0,7,7.0,8,8.0,9,9.0);\
            }",
        ),
        (
            "binary80",
            "int f(int n,...){\
                va_list a;\
                long double x,y;\
                int z;\
                va_start(a,n);\
                x=va_arg(a,long double);\
                z=va_arg(a,int);\
                y=va_arg(a,long double);\
                return x==3.25L&&z==7&&y==-4.5L;\
            }\
            int main(void){\
                return f(3,3.25L,7,-4.5L);\
            }",
        ),
        (
            "aligned_overflow",
            "int f(int a,int b,int c,int d,int e,int n,...){\
                va_list v;\
                int x,y;\
                long double z;\
                va_start(v,n);\
                x=va_arg(v,int);\
                z=va_arg(v,long double);\
                y=va_arg(v,int);\
                return x==7&&z==8.5L&&y==9;\
            }\
            int main(void){\
                return f(1,2,3,4,5,6,7,8.5L,9);\
            }",
        ),
        (
            "named_stack",
            "int f(int a,int b,int c,int d,int e,int f,int g,int n,...){\
                va_list v;\
                int x;\
                va_start(v,n);\
                x=va_arg(v,int);\
                return x==9&&v[0].gp_offset==48;\
            }\
            int main(void){\
                return f(1,2,3,4,5,6,7,8,9);\
            }",
        ),
        (
            "named_binary80",
            "int f(long double n,...){\
                va_list a;\
                long double x;\
                int y;\
                va_start(a,n);\
                x=va_arg(a,long double);\
                y=va_arg(a,int);\
                return n==1.0L&&x==2.0L&&y==3;\
            }\
            int main(void){\
                return f(1.0L,2.0L,3);\
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
