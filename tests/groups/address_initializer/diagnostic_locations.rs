// Diagnostic locations cases for address initializer.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Delayed fixup errors must point to the initializer, not the last function.
    for (name, source, diagnostic) in [
        (
            "unresolved",
            "#line 120 \"address_origin.c\"\n\
            extern int missing;\n\
            int *p=&missing;\n\
            int main(void){\
                return 0;\
            }\n",
            "address_origin.c:121:8:lcc: E009",
        ),
        (
            "range",
            "#line 220 \"address_range.c\"\n\
            char a;\n\
            char *p=&a+(-9223372036854775807L-1);\n\
            int main(void){\
                return 0;\
            }\n",
            "address_range.c:221:11:lcc: E011",
        ),
    ] {
        cases.push(Case::new(
            format!("address_initializer_origin_{name}"),
            "delayed data relocation error retains source position and category",
            source,
            Expectation::Reject(diagnostic.into()),
        ));
    }
}
