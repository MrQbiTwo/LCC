// Invalid initializers cases for conversion constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "initializer_static_nonzero",
            "static pointer initialization does not make a nonzero integer valid",
            r#"static int *pointer = 1;
int main(void) {
    return 0;
}
"#,
        ),
        (
            "initializer_braced_nonzero",
            "a scalar brace initializer still requires a valid pointer conversion",
            r#"int main(void) {
    int *pointer = {1};
    return 0;
}
"#,
        ),
        (
            "initializer_const_zero",
            "a const integer object is not a null pointer constant in C",
            r#"int main(void) {
    const int zero = 0;
    int *pointer = zero;
    return 0;
}
"#,
        ),
        (
            "initializer_array_element",
            "every pointer array element receives the implicit conversion check",
            r#"int main(void) {
    int *pointers[2] = {0, 0.0};
    return 0;
}
"#,
        ),
        (
            "initializer_structure_member",
            "pointer structure members preserve immediate pointee qualifiers",
            r#"struct View { int *pointer; };
int main(void) {
    const int value = 0;
    struct View view = {&value};
    return 0;
}
"#,
        ),
        (
            "initializer_union_member",
            "the initialized union member receives the implicit conversion check",
            r#"union View { int *pointer; long value; };
int main(void) {
    union View view = {1};
    return 0;
}
"#,
        ),
        (
            "initializer_pointer_integer",
            "an automatic integer initializer cannot implicitly consume a pointer",
            r#"int main(void) {
    int value = 0;
    long address = &value;
    return 0;
}
"#,
        ),
        (
            "initializer_discard_qualifiers",
            "void-to-object initialization must retain the source pointee qualifiers",
            r#"int main(void) {
    const void *opaque = 0;
    int *pointer = opaque;
    return 0;
}
"#,
        ),
        (
            "initializer_static_qualifier",
            "static address initializers cannot discard pointee qualifiers",
            r#"static const int value = 0;
static int *pointer = &value;
int main(void) {
    return 0;
}
"#,
        ),
        (
            "initializer_array_bound",
            "pointer initializers require compatible known array bounds",
            r#"int main(void) {
    int values[3] = {0, 0, 0};
    int (*pointer)[2] = &values;
    return 0;
}
"#,
        ),
        (
            "initializer_function_object",
            "function designators do not implicitly initialize void object pointers",
            r#"int identity(int value) {
    return value;
}
int main(void) {
    void *opaque = identity;
    return 0;
}
"#,
        ),
        (
            "initializer_record_value",
            "a record copy initializer cannot use a different record tag",
            r#"struct Left { int value; };
struct Right { int value; };
int main(void) {
    struct Right right = {0};
    struct Left left = right;
    return 0;
}
"#,
        ),
        (
            "initializer_void_value",
            "void-valued expressions cannot initialize arithmetic objects",
            r#"void nothing(void) {}
int main(void) {
    int value = nothing();
    return 0;
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("conversion_reject_{suffix}"),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
