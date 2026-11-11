// Addresses and layout cases for tentative.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "address_before",
            "an address constant can refer to the incomplete tentative object",
            "struct S object;\
            struct S *pointer=&object;\
            struct S{\
                long n;\
            };\
            int main(void){\
                pointer->n=61;\
                return !(pointer==&object&&object.n==61);\
            }",
        ),
        (
            "qualified_address_before",
            "qualified pointer relocations survive target completion",
            "struct S;\
            const struct S object;\
            const struct S *pointer=&object;\
            struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                return !(pointer==&object&&pointer->n==0&&sizeof(*pointer)==16);\
            }",
        ),
        (
            "function_address_before",
            "a pre-completion function can return the object's address",
            "struct S object;\
            struct S *get(void){\
                return &object;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                get()->n=67;\
                return !(get()==&object&&object.n==67);\
            }",
        ),
        (
            "parenthesized_address",
            "parentheses do not turn an incomplete address operand into a value use",
            "struct S object;\
            struct S *get(void){\
                return &(object);\
            }\
            struct S{\
                long n;\
            };\
            int main(void){\
                get()->n=71;\
                return !(get()==&object&&object.n==71);\
            }",
        ),
        (
            "address_dereference",
            "address-of an incomplete pointer dereference needs no object value",
            "struct S object;\
            struct S *identity(struct S *p){\
                return &*p;\
            }\
            struct S{\
                long n;\
            };\
            int main(void){\
                identity(&object)->n=73;\
                return !(identity(&object)==&object&&object.n==73);\
            }",
        ),
        (
            "qualified_address_dereference",
            "qualified incomplete pointer dereference may still supply an address",
            "struct S;\
            const struct S object;\
            const struct S *identity(const struct S *p){\
                return &(*p);\
            }\
            struct S{\
                long n;\
            };\
            int main(void){\
                return !(identity(&object)==&object&&identity(&object)->n==0);\
            }",
        ),
        (
            "block_extern",
            "block extern binds the incomplete file-scope object",
            "struct S object;\
            struct S *get(void){\
                extern struct S object;\
                return &object;\
            }\
            struct S{\
                long n;\
            };\
            int main(void){\
                get()->n=71;\
                return !(get()==&object&&object.n==71);\
            }",
        ),
        (
            "block_extern_first",
            "a hidden block extern and later tentative definition share identity",
            "struct S;\
            struct S *get(void){\
                extern struct S object;\
                return &object;\
            }\
            struct S object;\
            struct S{\
                long n;\
            };\
            int main(void){\
                get()->n=73;\
                return !(get()==&object&&object.n==73);\
            }",
        ),
        (
            "qualified_block_extern_first",
            "a qualified hidden block entity is exposed by a later tentative definition",
            "struct S;\
            const struct S *get(void){\
                extern const struct S object;\
                return &object;\
            }\
            const struct S object;\
            struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                return !(get()==&object&&get()->c==0&&get()->n==0&&sizeof(object)==16);\
            }",
        ),
        (
            "tag_shadow",
            "a block tag shadow does not replace the file-scope object's type",
            "struct S object;\
            int f(void){\
                struct S{\
                    char c;\
                };\
                struct S local;\
                local.c=3;\
                return sizeof(local)+local.c;\
            }\
            struct S{\
                long n;\
                long m;\
            };\
            int main(void){\
                object.n=79;\
                return !(sizeof(object)==16&&object.n==79&&object.m==0&&f()==4);\
            }",
        ),
        (
            "typedef_shadow_extern",
            "a typedef keeps block extern tied to the outer tag despite a local shadow",
            "typedef struct S T;\
            T object;\
            T *get(void){\
                struct S{\
                    char c;\
                };\
                extern T object;\
                return &object;\
            }\
            struct S{\
                long n;\
            };\
            int main(void){\
                get()->n=83;\
                return !(get()==&object&&object.n==83);\
            }",
        ),
        (
            "recursive",
            "the completed tentative record retains self pointers",
            "struct Node root;\
            struct Node{\
                int n;\
                struct Node *next;\
            };\
            int main(void){\
                root.n=89;\
                root.next=&root;\
                return root.next->n!=89;\
            }",
        ),
        (
            "nested_layout",
            "nested completed fields determine final tentative layout",
            "struct Outer object;\
            struct Inner{\
                char c;\
                long n;\
            };\
            struct Outer{\
                char head;\
                struct Inner inner;\
                double tail;\
            };\
            int main(void){\
                object.inner.n=97;\
                object.tail=1.5;\
                return !(sizeof(object)==32&&object.head==0&&object.inner.c==0&&object.inner.n==97&&\
                object.tail==1.5);\
            }",
        ),
        (
            "bit_fields",
            "late-completed bit fields receive zeroed storage",
            "struct S object;\
            struct S{\
                unsigned a:3;\
                signed int b:5;\
                long n;\
            };\
            int main(void){\
                int zero=object.a==0&&object.b==0&&object.n==0;\
                object.a=5;\
                object.b=-3;\
                object.n=101;\
                return !(zero&&object.a==5&&object.b==-3&&object.n==101);\
            }",
        ),
        (
            "array_member",
            "late record size includes its complete array members",
            "struct S object;\
            struct S{\
                int a[3];\
                char c;\
            };\
            int main(void){\
                object.a[2]=103;\
                return !(sizeof(object)==16&&object.a[0]==0&&object.a[1]==0&&object.a[2]==103&&object.c==0);\
            }",
        ),
        (
            "binary80_alignment",
            "late record alignment includes binary80 members",
            "char lead;\
            struct S object;\
            struct S{\
                char c;\
                long double n;\
            };\
            int main(void){\
                object.n=2.5L;\
                return !(sizeof(object)==32&&(unsigned long)&object%16==0&&object.c==0&&object.n==2.5L);\
            }",
        ),
] {
        cases.push(Case::new(
format!("tentative_{id}"),
purpose,
source,
Expectation::Exit(0),
));
    }
}
