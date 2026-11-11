// Offset constraints cases for standard headers.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    let stddef = "#include <stddef.h>";
    for (id, purpose, source) in [
        (
            "offset_reject_scalar",
            "offsetof requires a structure or union type",
            "int main(void){return offsetof(int,n);}",
        ),
        (
            "offset_reject_pointer_type",
            "a pointer type is not a record type",
            "struct S{int n;};int main(void){return offsetof(struct S*,n);}",
        ),
        (
            "offset_reject_incomplete",
            "offsetof rejects an incomplete record",
            "struct S;int main(void){return offsetof(struct S,n);}",
        ),
        (
            "offset_reject_missing",
            "offsetof rejects an unknown member name",
            "struct S{int n;};int main(void){return offsetof(struct S,missing);}",
        ),
        (
            "offset_reject_bitfield",
            "a bit-field has no independently addressable byte offset",
            "struct S{unsigned n:3;};int main(void){return offsetof(struct S,n);}",
        ),
        (
            "offset_reject_nested_bitfield",
            "a nested path cannot terminate in a bit-field",
            "struct I{\
                unsigned n:3;\
            };\
            struct S{\
                struct I i;\
            };\
            int main(void){\
                return offsetof(struct S,i.n);\
            }",
        ),
        (
            "offset_reject_scalar_member",
            "a dot path cannot continue through a scalar member",
            "struct S{int n;};int main(void){return offsetof(struct S,n.x);}",
        ),
        (
            "offset_reject_missing_nested",
            "each nested member name must exist on its owning record",
            "struct I{\
                int n;\
            };\
            struct S{\
                struct I i;\
            };\
            int main(void){\
                return offsetof(struct S,i.missing);\
            }",
        ),
        (
            "offset_reject_scalar_index",
            "an index requires an array member",
            "struct S{int n;};int main(void){return offsetof(struct S,n[0]);}",
        ),
        (
            "offset_reject_pointer_index",
            "a pointer member cannot supply a constant offset to pointee storage",
            "struct S{int *p;};int main(void){return offsetof(struct S,p[0]);}",
        ),
        (
            "offset_reject_negative",
            "negative member-array indices are outside the supported designator domain",
            "struct S{int a[3];};int main(void){return offsetof(struct S,a[-1]);}",
        ),
        (
            "offset_reject_past_end",
            "one-past array indices do not designate a member subobject",
            "struct S{int a[3];};int main(void){return offsetof(struct S,a[3]);}",
        ),
        (
            "offset_reject_unsigned_range",
            "an oversized unsigned index cannot wrap into a valid array position",
            "struct S{int a[3];};int main(void){return offsetof(struct S,a[18446744073709551615UL]);}",
        ),
        (
            "offset_reject_inner_range",
            "each dimension validates its own array bound",
            "struct S{int a[2][3];};int main(void){return offsetof(struct S,a[0][3]);}",
        ),
        (
            "offset_reject_variable",
            "a member-array index must be an integer constant expression",
            "struct S{int a[3];};int main(void){int n=1;return offsetof(struct S,a[n]);}",
        ),
        (
            "offset_reject_call",
            "a function call cannot provide a member-array constant index",
            "int f(void){\
                return 1;\
            }\
            struct S{\
                int a[3];\
            };\
            int main(void){\
                return offsetof(struct S,a[f()]);\
            }",
        ),
        (
            "offset_reject_float",
            "a floating constant is not an integer array designator",
            "struct S{int a[3];};int main(void){return offsetof(struct S,a[1.0]);}",
        ),
        (
            "offset_reject_postfix",
            "the builtin result reaches postfix parsing and cannot be incremented",
            "struct S{char c;long n;};int main(void){return offsetof(struct S,n)++;}",
        ),
    ] {
        cases.push(Case::new(
            format!("standard_headers_{id}"),
            purpose,
            format!("{stddef}\n{source}"),
            Expectation::Reject("E008".into()),
        ));
    }
    cases.push(Case::new(
        "standard_headers_offset_reject_index_overflow",
        "arithmetic overflow in a member index retains the constant evaluator's range diagnostic",
        format!(
            "{stddef}\n\
            struct S{{\
                int a[3];\
            }};\
            int main(void){{\
                return offsetof(struct S,a[9223372036854775807L+1L]);\
            }}"
        ),
        Expectation::Reject("E011".into()),
    ));
}
