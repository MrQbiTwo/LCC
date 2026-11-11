// Target policy cases for type identity.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // These represent LCC's chosen implementation-defined types; they are
    // deliberately excluded from cross-compiler C89 consensus totals.
    cases.push(Case::new(
        "type_identity_target_positive_enum_int",
        "LCC chooses signed int compatibility even for a nonnegative-only enum",
        r#"enum Value { ZERO = 0, POSITIVE = 59 };
int main(void) {
    enum Value value = POSITIVE;
    int *integer = &value;
    enum Value *typed = integer;
    return *integer != 59 || typed != &value;
}
"#,
        Expectation::Exit(0),
    ));
    cases.push(Case::new(
        "type_identity_target_signed_plain_char",
        "LCC plain char remains a signed byte despite its separate type identity",
        r#"int main(void) {
    char plain = -1;
    signed char explicit_signed = -1;
    unsigned char explicit_unsigned = 255;
    return !(sizeof(plain) == 1 && plain < 0 && explicit_signed < 0 &&
             explicit_unsigned == 255);
}
"#,
        Expectation::Exit(0),
    ));
}
