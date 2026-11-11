// Offset layout cases for standard headers.
use super::pass;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    let stddef = "#include <stddef.h>";
    for (id, purpose, source) in [
        (
            "offset_padding",
            "offsetof follows target alignment and internal padding",
            "struct S{\
                char c;\
                short s;\
                int i;\
                long l;\
                double d;\
                char z;\
            };\
            int main(void){\
                return !(offsetof(struct S,c)==0&&offsetof(struct S,s)==2&&offsetof(struct S,i)==4&&\
                offsetof(struct S,l)==8&&offsetof(struct S,d)==16&&offsetof(struct S,z)==24);\
            }",
        ),
        (
            "offset_extended",
            "long double members retain sixteen-byte alignment",
            "struct S{\
                char c;\
                long double x;\
                char z;\
            };\
            int main(void){\
                return !(offsetof(struct S,x)==16&&offsetof(struct S,z)==32);\
            }",
        ),
        (
            "offset_union",
            "all direct union members begin at zero",
            "union U{\
                char c;\
                long n;\
                long double x;\
                int a[3];\
            };\
            int main(void){\
                return !(offsetof(union U,c)==0&&offsetof(union U,n)==0&&offsetof(union U,x)==0&&\
                offsetof(union U,a)==0);\
            }",
        ),
        (
            "offset_nested",
            "nested structure and union paths accumulate each member offset",
            "struct I{\
                short a;\
                long b;\
            };\
            union U{\
                int i;\
                struct I s;\
            };\
            struct O{\
                char x;\
                union U u;\
                char z;\
            };\
            int main(void){\
                return !(offsetof(struct O,u.s.b)==16&&offsetof(struct O,z)==24);\
            }",
        ),
        (
            "offset_array",
            "array designators include the element size and enclosing member offset",
            "struct S{\
                char c;\
                short a[4];\
                long z;\
            };\
            int main(void){\
                return !(offsetof(struct S,a)==2&&offsetof(struct S,a[0])==2&&offsetof(struct S,a[3])==8&&\
                offsetof(struct S,z)==16);\
            }",
        ),
        (
            "offset_array_records",
            "a record member path can continue after a constant array index",
            "struct I{\
                char c;\
                long n;\
            };\
            struct S{\
                short x;\
                struct I a[3];\
                long tail;\
            };\
            int main(void){\
                return !(offsetof(struct S,a[2].n)==48&&offsetof(struct S,tail)==56);\
            }",
        ),
        (
            "offset_multidimensional",
            "each array dimension contributes its own complete element stride",
            "struct S{\
                char x;\
                int a[2][3];\
                char z;\
            };\
            int main(void){\
                return !(offsetof(struct S,a[1][2])==24&&offsetof(struct S,z)==28);\
            }",
        ),
        (
            "offset_index_expression",
            "array indices accept integer constant expressions and enumeration values",
            "enum E{\
                N=2};\
            struct S{\
                char c;\
                int a[5];\
            };\
            int main(void){\
                return !(offsetof(struct S,a[N+1])==16&&offsetof(struct S,a[sizeof(short)])==12);\
            }",
        ),
        (
            "offset_nested_builtin",
            "a nested offsetof expression can supply a constant array index",
            "struct I{\
                char c;\
                char index;\
            };\
            struct S{\
                int a[3];\
            };\
            int main(void){\
                return offsetof(struct S,a[offsetof(struct I,index)])!=4;\
            }",
        ),
        (
            "offset_qualified_alias",
            "qualified record typedefs retain the canonical member layout",
            "struct S{\
                char c;\
                long n;\
            };\
            typedef const volatile struct S T;\
            int main(void){\
                return offsetof(T,n)!=8;\
            }",
        ),
        (
            "offset_anonymous_alias",
            "an unnamed record can be addressed through its typedef name",
            "typedef struct{char c;long n;} T;int main(void){return offsetof(T,n)!=8;}",
        ),
        (
            "offset_completed_tag",
            "a forward declared record is usable after its definition completes",
            "typedef struct S T;struct S{char c;long n;};int main(void){return offsetof(T,n)!=8;}",
        ),
        (
            "offset_qualified_forward",
            "a qualified alias observes completion of its previously incomplete record",
            "typedef const volatile struct S T;\
            struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                return offsetof(T,n)!=8;\
            }",
        ),
        (
            "offset_function_pointer",
            "a pointer-to-function member has ordinary pointer alignment",
            "struct S{char c;int (*f)(int);};int main(void){return offsetof(struct S,f)!=8;}",
        ),
        (
            "offset_after_bitfields",
            "ordinary members remain addressable after packed bit-fields",
            "struct S{\
                unsigned a:3;\
                unsigned b:5;\
                char c;\
                int d;\
            };\
            int main(void){\
                return !(offsetof(struct S,c)==1&&offsetof(struct S,d)==4);\
            }",
        ),
] {
        cases.push(pass(id, purpose, stddef, source));
    }
}
