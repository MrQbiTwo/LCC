// Constraints cases for vararg.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, source) in [
        (
            "nonvariadic",
            "int main(void){va_list a;va_start(a,a);return 0;}",
        ),
        (
            "wrong_last",
            "int f(int x,int n,...){va_list a;va_start(a,x);return 0;}",
        ),
        (
            "shadow_last",
            "int f(int n,...){va_list a;{int n;va_start(a,n);}return 0;}",
        ),
        (
            "last_expression",
            "int f(int n,...){va_list a;va_start(a,n+0);return 0;}",
        ),
        (
            "const_list",
            "int f(int n,...){const va_list a;va_start(a,n);return 0;}",
        ),
        (
            "bad_list",
            "int f(int n,...){int a;va_start(a,n);return 0;}",
        ),
        (
            "char",
            "int f(int n,...){va_list a;va_start(a,n);return va_arg(a,char);}",
        ),
        (
            "short",
            "int f(int n,...){va_list a;va_start(a,n);return va_arg(a,short);}",
        ),
        (
            "float",
            "int f(int n,...){va_list a;va_start(a,n);return va_arg(a,float);}",
        ),
        (
            "void",
            "int f(int n,...){va_list a;va_start(a,n);va_arg(a,void);return 0;}",
        ),
        (
            "incomplete",
            "struct S;int f(int n,...){va_list a;va_start(a,n);va_arg(a,struct S);return 0;}",
        ),
        (
            "array",
            "int f(int n,...){va_list a;va_start(a,n);va_arg(a,int[2]);return 0;}",
        ),
        (
            "function",
            "int f(int n,...){va_list a;va_start(a,n);va_arg(a,int(void));return 0;}",
        ),
        (
            "copy_source",
            "int f(int n,...){va_list a;int x;va_copy(a,x);return 0;}",
        ),
    ] {
        cases.push(Case::new(
            format!("vararg_reject_{name}"),
            format!("invalid stdarg use {name}"),
            format!("#include <stdarg.h>\n{source}"),
            Expectation::Reject("E008".into()),
        ));
    }
}
