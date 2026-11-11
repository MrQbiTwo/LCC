// Incomplete values cases for tentative.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "reject_argument",
            "a by-value function argument requires a currently complete record",
            "struct S object;\
            void take(struct S);\
            int f(void){\
                take(object);\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            void take(struct S x){}int main(void){\
                return f();\
            }",
        ),
        (
            "reject_unprototyped_argument",
            "an unspecified parameter list does not permit incomplete record values",
            "struct S object;\
            void take();\
            int f(void){\
                take(object);\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            void take(struct S x){}int main(void){\
                return f();\
            }",
        ),
        (
            "reject_discarded",
            "a discarded expression still requires a complete record value",
            "struct S object;int f(void){object;return 0;}struct S{int n;};int main(void){return f();}",
        ),
        (
            "reject_discarded_volatile",
            "a discarded volatile record cannot wait for tag completion",
            "struct S;\
            volatile struct S object;\
            int f(void){\
                object;\
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
            "reject_comma_left",
            "the left comma operand consumes its incomplete record value",
            "struct S object;int f(void){return object,0;}struct S{int n;};int main(void){return f();}",
        ),
        (
            "reject_comma_right",
            "a discarded comma result cannot carry an incomplete record value",
            "struct S object;\
            int f(void){\
                0,object;\
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
            "reject_conditional",
            "conditional record arms must be complete before selection",
            "struct S a,b;int f(void){1?a:b;return 0;}struct S{int n;};int main(void){return f();}",
        ),
        (
            "reject_void_cast",
            "conversion to void still consumes the incomplete record operand",
            "struct S object;\
            int f(void){\
                (void)object;\
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
            "reject_sizeof_void_cast",
            "unevaluated sizeof does not excuse an invalid incomplete value conversion",
            "struct S object;\
            int f(void){\
                return sizeof((void)object,1);\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_for_initializer",
            "a discarded for initializer requires a complete record value",
            "struct S object;\
            int f(void){\
                for(object;0;){}return 0;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_for_increment",
            "an unexecuted for increment cannot consume an incomplete record value",
            "struct S object;\
            int f(void){\
                for(;0;object){}return 0;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_extern_assignment",
            "declaration-only objects also require completeness at an assignment",
            "extern struct S a,b;\
            int f(void){\
                a=b;\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            struct S a,b;\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_extern_discarded",
            "declaration-only objects cannot supply incomplete discarded values",
            "extern struct S object;\
            int f(void){\
                object;\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            struct S object;\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_extern_void_cast",
            "declaration-only objects cannot hide incomplete values in void casts",
            "extern struct S object;\
            int f(void){\
                (void)object;\
                return 0;\
            }\
            struct S{\
                int n;\
            };\
            struct S object;\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_extern_difference",
            "declaration-only object addresses still need a complete subtraction stride",
            "extern struct S object;\
            int f(void){\
                return &object-&object;\
            }\
            struct S{\
                int n;\
            };\
            struct S object;\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_pointer_stride",
            "pointer arithmetic cannot use a not-yet-complete record size",
            "struct S object;struct S *p=&object+1;struct S{int n;};int main(void){return 0;}",
        ),
        (
            "reject_pointer_difference",
            "pointer difference requires complete element types before folding",
            "struct S object;\
            int f(void){\
                return &object-&object;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_qualified_pointer_difference",
            "qualified pointer difference cannot wait for tag completion",
            "struct S;\
            const struct S object;\
            int f(void){\
                return &object-&object;\
            }\
            struct S{\
                int n;\
            };\
            int main(void){\
                return f();\
            }",
        ),
        (
            "reject_array_element",
            "an array declarator cannot defer an incomplete element type",
            "struct S objects[2];struct S{int n;};int main(void){return 0;}",
        ),
        (
            "reject_unknown_array_element",
            "an unknown array bound does not permit an incomplete element type",
            "struct S objects[];struct S{int n;};int main(void){return 0;}",
        ),
        (
            "reject_qualified_array_element",
            "qualified incomplete elements still reject array construction",
            "struct S;const struct S objects[2];struct S{int n;};int main(void){return 0;}",
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
