// List lifetime cases for vararg.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, source) in [
        (
            "copy",
            "int f(int n,...){\
                va_list a,b;\
                int x,y,z;\
                va_start(a,n);\
                x=va_arg(a,int);\
                va_copy(b,a);\
                y=va_arg(a,int);\
                z=va_arg(b,int);\
                va_end(a);\
                va_end(b);\
                return x==3&&y==4&&z==4;\
            }\
            int main(void){\
                return f(2,3,4);\
            }",
        ),
        (
            "restart",
            "int f(int n,...){\
                va_list a;\
                int x,y;\
                va_start(a,n);\
                x=va_arg(a,int);\
                va_end(a);\
                va_start(a,n);\
                y=va_arg(a,int);\
                return x==7&&y==7;\
            }\
            int main(void){\
                return f(1,7);\
            }",
        ),
        (
            "two_lists",
            "int f(int n,...){\
                va_list a,b;\
                int x,y;\
                va_start(a,n);\
                va_start(b,n);\
                x=va_arg(a,int);\
                y=va_arg(b,int);\
                return x==7&&y==7;\
            }\
            int main(void){\
                return f(1,7);\
            }",
        ),
        (
            "helper",
            "int g(va_list a){\
                return va_arg(a,int);\
            }\
            int f(int n,...){\
                va_list a;\
                int x,y;\
                va_start(a,n);\
                x=g(a);\
                y=va_arg(a,int);\
                return x==3&&y==4;\
            }\
            int main(void){\
                return f(2,3,4);\
            }",
        ),
        (
            "pointer_to_list",
            "int g(va_list *a){\
                return va_arg(*a,int);\
            }\
            int f(int n,...){\
                va_list a;\
                int x,y;\
                va_start(a,n);\
                x=g(&a);\
                y=g(&a);\
                return x==3&&y==4;\
            }\
            int main(void){\
                return f(2,3,4);\
            }",
        ),
        (
            "single_evaluation",
            "int k;\
            va_list *pick(va_list *p){\
                k++;\
                return p;\
            }\
            int f(int n,...){\
                va_list a,b;\
                int x;\
                va_start(*pick(&a),n);\
                va_copy(*pick(&b),*pick(&a));\
                x=va_arg(*pick(&b),int);\
                va_end(*pick(&a));\
                return x==7&&k==5;\
            }\
            int main(void){\
                return f(1,7);\
            }",
        ),
        (
            "nested",
            "int f(int n,...){\
                va_list a;\
                int x,y;\
                va_start(a,n);\
                x=va_arg(a,int);\
                if(n)y=f(0,3);\
                else y=4;\
                return x+y;\
            }\
            int main(void){\
                return f(1,2)==9;\
            }",
        ),
        (
            "list_layout",
            "int f(int n,...){\
                va_list a;\
                va_start(a,n);\
                return sizeof(va_list)==24&&a[0].gp_offset==8&&a[0].fp_offset==48;\
            }\
            int main(void){\
                return f(0);\
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
