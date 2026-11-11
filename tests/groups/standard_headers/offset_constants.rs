// Offset constants cases for standard headers.
use super::pass;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    let stddef = "#include <stddef.h>";
    for (id, purpose, source) in [
        (
            "offset_static_initializer",
            "offsetof initializes static scalar storage without runtime evaluation",
            "struct S{\
                char c;\
                long n;\
            };\
            size_t offset=offsetof(struct S,n);\
            int main(void){\
                static size_t local=offsetof(struct S,n);\
                return !(offset==8&&local==8);\
            }",
        ),
        (
            "offset_enum",
            "offsetof is an integer constant expression in enumerator definitions",
            "struct S{\
                char c;\
                long n;\
            };\
            enum E{\
                N=offsetof(struct S,n),M=N+1};\
            int main(void){\
                return !(N==8&&M==9);\
            }",
        ),
        (
            "offset_array_bound",
            "offsetof is an integer constant expression in file and block array bounds",
            "struct S{\
                char c;\
                long n;\
            };\
            char a[offsetof(struct S,n)];\
            int main(void){\
                char b[offsetof(struct S,n)+1];\
                return !(sizeof(a)==8&&sizeof(b)==9);\
            }",
        ),
        (
            "offset_typedef_bounds",
            "nested type parsing preserves a typedef's comma declarator storage class",
            "struct S{\
                char c;\
                long n;\
            };\
            typedef char A[offsetof(struct S,n)],B[offsetof(struct S,n)+1];\
            int main(void){\
                A a;\
                B b;\
                return !(sizeof(a)==8&&sizeof(b)==9);\
            }",
        ),
        (
            "offset_static_bounds",
            "nested type parsing preserves static storage on comma-separated local arrays",
            "struct S{\
                char c;\
                long n;\
            };\
            int f(void){\
                static int a[offsetof(struct S,n)]={0},b[offsetof(struct S,n)]={0};\
                a[0]++;\
                b[0]+=2;\
                return a[0]+b[0];\
            }\
            int main(void){\
                int a=f();\
                int b=f();\
                return !(a==3&&b==6);\
            }",
        ),
        (
            "offset_switch",
            "offsetof can label a switch case",
            "struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                switch(8){\
                    case offsetof(struct S,n):return 0;\
                    default:return 1;\
                }}",
        ),
        (
            "offset_pointer_initializer",
            "offsetof can be folded into a static address initializer displacement",
            "struct S{\
                char c;\
                long n;\
            };\
            char a[16];\
            char *p=a+offsetof(struct S,n);\
            int main(void){\
                return p!=a+8;\
            }",
        ),
        (
            "offset_result_type",
            "offsetof returns size_t with unsigned-long arithmetic",
            "struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                return !(sizeof(offsetof(struct S,n))==sizeof(size_t)&&offsetof(struct S,c)-1L>0);\
            }",
        ),
        (
            "offset_macro_path",
            "the member designator is macro-expanded before builtin parsing",
            "#define MEMBER a[1].n\n\
            struct I{\
                char c;\
                long n;\
            };\
            struct S{\
                char c;\
                struct I a[2];\
            };\
            int main(void){\
                return offsetof(struct S,MEMBER)!=32;\
            }",
        ),
        (
            "offset_local_tag",
            "offsetof resolves the record visible at its lexical use site",
            "struct S{long n;};int main(void){struct S{char c;long n;};return offsetof(struct S,n)!=8;}",
        ),
        (
            "offset_parentheses",
            "the offsetof expansion remains a single operand in larger expressions",
            "struct S{char c;long n;};int main(void){return 3*offsetof(struct S,n)+1!=25;}",
        ),
        (
            "offset_name_collision",
            "an ordinary identifier of the builtin name length remains ordinary",
            "int ordinary_value=7;int main(void){return ordinary_value!=7;}",
        ),
    ] {
        cases.push(pass(id, purpose, stddef, source));
    }
}
