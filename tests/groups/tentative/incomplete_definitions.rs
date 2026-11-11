// Incomplete definitions cases for tentative.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Deferral applies to external-linkage tentative objects only. Layout uses
    // and definitions which require an immediate complete type still fail.
    for (id, purpose, source) in [
        (
            "reject_final_struct",
            "an unresolved tentative struct fails at translation-unit end",
            "struct S object;int main(void){return 0;}",
        ),
        (
            "reject_final_union",
            "an unresolved tentative union fails at translation-unit end",
            "union U object;int main(void){return 0;}",
        ),
        (
            "reject_final_const",
            "an unresolved qualified tentative object fails at translation-unit end",
            "struct S;const struct S object;int main(void){return 0;}",
        ),
        (
            "reject_final_typedef",
            "an unresolved tentative typedef object fails at translation-unit end",
            "typedef union U T;volatile T object;int main(void){return 0;}",
        ),
        (
            "reject_inner_completion",
            "a block tag definition cannot complete an outer tentative object",
            "struct S object;int main(void){struct S{int n;};return 0;}",
        ),
        (
            "reject_initialized_struct",
            "an initialized struct requires a complete type at its declaration",
            "struct S object={0};struct S{int n;};int main(void){return 0;}",
        ),
        (
            "reject_initialized_union",
            "an initialized union requires a complete type at its declaration",
            "union U object={0};union U{long n;};int main(void){return 0;}",
        ),
        (
            "reject_extern_initialized",
            "extern with an initializer is an immediate definition",
            "extern struct S object={0};struct S{int n;};int main(void){return 0;}",
        ),
        (
            "reject_auto",
            "an automatic record cannot wait for a later file-scope completion",
            "struct S;\
            int f(void){\
                struct S object;\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_block_static",
            "a block static record cannot wait for later tag completion",
            "struct S;\
            int f(void){\
                static struct S object;\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_static_struct",
            "an internal-linkage tentative struct needs an immediate complete type",
            "static struct S object;struct S{int n;};int main(void){return 0;}",
        ),
        (
            "reject_static_const",
            "an internal-linkage qualified tentative struct needs a complete type",
            "struct S;static const struct S object;struct S{long n;};int main(void){return 0;}",
        ),
        (
            "reject_static_union",
            "an internal-linkage tentative union needs an immediate complete type",
            "static union U object;union U{long n;};int main(void){return 0;}",
        ),
        (
            "reject_sizeof",
            "sizeof before tag completion is not deferred",
            "struct S object;\
            int f(void){\
                return sizeof(object);\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_qualified_sizeof",
            "qualified forward copies remain incomplete before their tag completes",
            "struct S;\
            const struct S object;\
            int f(void){\
                return sizeof(object);\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_member",
            "member access before tag completion is rejected",
            "struct S object;int f(void){return object.n;}struct S{int n;};int main(void){return f();}",
        ),
        (
            "reject_assignment",
            "aggregate assignment requires completion when the expression is parsed",
            "struct S a,b;int f(void){a=b;return 0;}struct S{int n;};int main(void){return f();}",
        ),
        (
            "reject_volatile_assignment",
            "qualified aggregate assignment cannot defer completeness",
            "struct S;\
            volatile struct S a,b;\
            int f(void){\
                a=b;\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
] {
        cases.push(Case::new(
            format!("tentative_{id}"),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
