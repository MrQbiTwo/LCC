// Stddef cases for standard headers.
use super::pass;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    let stddef = "#include <stddef.h>";
    for (id, purpose, source) in [
        (
            "stddef_widths",
            "stddef typedefs follow the Linux x86-64 LP64 model",
            "int main(void){return !(sizeof(size_t)==8&&sizeof(ptrdiff_t)==8&&sizeof(wchar_t)==4);}",
        ),
        (
            "size_type",
            "size_t is compatible with unsigned long in function declarations",
            "unsigned long f(size_t);\
            size_t f(unsigned long x){\
                return x;\
            }\
            int main(void){\
                size_t n=(size_t)-1;\
                return !(f(n)==18446744073709551615UL&&n>0);\
            }",
        ),
        (
            "difference_type",
            "ptrdiff_t is compatible with signed long and retains negative differences",
            "long f(ptrdiff_t);\
            ptrdiff_t f(long x){\
                return x;\
            }\
            int main(void){\
                int a[4];\
                ptrdiff_t n=&a[0]-&a[3];\
                return !(f(n)==-3&&(ptrdiff_t)-1<0);\
            }",
        ),
        (
            "wide_type",
            "wchar_t is compatible with signed int",
            "int f(wchar_t);\
            wchar_t f(int x){\
                return x;\
            }\
            int main(void){\
                return !(f(-1)==-1&&(wchar_t)-1<0);\
            }",
        ),
        (
            "null_objects",
            "NULL initializes qualified and unqualified object pointers statically",
            "int *p=NULL;\
            const long *q=NULL;\
            void *r=NULL;\
            int main(void){\
                return !(p==0&&q==0&&r==0&&sizeof(NULL)==sizeof(void*));\
            }",
        ),
        (
            "null_function",
            "NULL is a null pointer constant for a function pointer initializer",
            "int (*p)(void)=NULL;int main(void){return p!=0;}",
        ),
        (
            "null_conditional",
            "NULL preserves an object pointer's type in a conditional expression",
            "int main(void){int x=7;int *p=1?&x:NULL;int *q=0?NULL:&x;return !(*p==7&&*q==7);}",
        ),
    ] {
        cases.push(pass(id, purpose, stddef, source));
    }
}
