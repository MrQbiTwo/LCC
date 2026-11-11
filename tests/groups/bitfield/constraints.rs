// Constraints cases for bitfield.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, purpose, source, code) in [
        (
            "negative",
            "negative field width",
            "struct S{int a:-1;};",
            "E008",
        ),
        (
            "too_wide",
            "field width exceeds declared type",
            "struct S{unsigned a:33;};",
            "E008",
        ),
        (
            "named_zero",
            "zero-width fields must be unnamed",
            "struct S{int a:0;};",
            "E008",
        ),
        (
            "floating_width",
            "field width must be an integer constant",
            "struct S{int a:2.5;};",
            "E008",
        ),
        (
            "nonconstant",
            "field width cannot read an object",
            "int n;struct S{int a:n;};",
            "E008",
        ),
        (
            "float_type",
            "floating bit-field base type",
            "struct S{float a:3;};",
            "E008",
        ),
        (
            "long_type",
            "unsupported non-C89 bit-field base type",
            "struct S{long a:3;};",
            "E008",
        ),
        (
            "pointer",
            "pointer bit-field declarator",
            "struct S{int *a:3;};",
            "E008",
        ),
        (
            "array",
            "array bit-field declarator",
            "struct S{int a[2]:3;};",
            "E008",
        ),
        (
            "duplicate",
            "duplicate field names remain invalid",
            "struct S{int a:3,a:4;};",
            "E009",
        ),
        (
            "address",
            "bit-fields do not have C object addresses",
            "struct S{int a:3;};int main(void){struct S s;return &s.a!=0;}",
            "E008",
        ),
        (
            "sizeof",
            "sizeof cannot be applied to a bit-field",
            "struct S{int a:3;};int main(void){struct S s;return sizeof(s.a);}",
            "E008",
        ),
        (
            "const_field",
            "a const bit-field cannot be assigned",
            "struct S{const int a:3;};int main(void){struct S s={1};s.a=2;return 0;}",
            "E008",
        ),
        (
            "const_base",
            "aggregate const qualification reaches bit-fields",
            "struct S{int a:3;};int main(void){const struct S s={1};s.a++;return 0;}",
            "E008",
        ),
        (
            "const_copy",
            "whole-object assignment respects const bit-field members",
            "struct S{const int a:3;};int main(void){struct S a={1},b={2};a=b;return 0;}",
            "E008",
        ),
        (
            "rvalue",
            "returned aggregate bit-fields are not modifiable lvalues",
            "struct S{\
                int a:3;\
            };\
            struct S f(void){\
                struct S s={1};\
                return s;\
            }\
            int main(void){\
                f().a=2;\
                return 0;\
            }",
            "E008",
        ),
        (
            "excess_init",
            "unnamed fields do not absorb excess initializer values",
            "struct S{unsigned :3;unsigned a:2;};struct S s={1,2};",
            "E006",
        ),
    ] {
        cases.push(Case::new(
            format!("bitfield_reject_{name}"),
            purpose,
            source,
            Expectation::Reject(code.into()),
        ));
    }
}
