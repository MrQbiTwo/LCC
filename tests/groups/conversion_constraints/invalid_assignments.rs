// Invalid assignments cases for conversion constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "assignment_nonzero",
            "integer pointer assignment requires a null pointer constant even in sizeof",
            r#"int main(void) {
    int *pointer = 0;
    return sizeof(pointer = 1);
}
"#,
        ),
        (
            "assignment_variable_zero",
            "a zero-valued variable is not a null pointer constant",
            r#"int main(void) {
    int zero = 0;
    int *pointer = 0;
    pointer = zero;
    return 0;
}
"#,
        ),
        (
            "assignment_float_zero",
            "floating zero cannot implicitly convert to an object pointer",
            r#"int main(void) {
    int *pointer = 0;
    if (0) {
        pointer = 0.0;
    }
    return 0;
}
"#,
        ),
        (
            "assignment_pointer_integer",
            "an object pointer cannot implicitly convert to an integer",
            r#"int main(void) {
    int value = 0;
    long address = 0;
    return sizeof(address = &value);
}
"#,
        ),
        (
            "assignment_pointer_float",
            "an object pointer cannot implicitly convert to a floating value",
            r#"int main(void) {
    int value = 0;
    double address = 0;
    address = &value;
    return 0;
}
"#,
        ),
        (
            "assignment_discard_const",
            "pointer assignment cannot discard immediate pointee const",
            r#"int main(void) {
    const int value = 3;
    int *pointer = 0;
    pointer = &value;
    return 0;
}
"#,
        ),
        (
            "assignment_discard_volatile",
            "pointer assignment cannot discard immediate pointee volatile",
            r#"int main(void) {
    volatile int value = 3;
    void *pointer = 0;
    pointer = &value;
    return 0;
}
"#,
        ),
        (
            "assignment_nested_qualifier",
            "adding nested pointee qualifiers is not an implicit pointer conversion",
            r#"int main(void) {
    int *pointer = 0;
    const int **nested = 0;
    nested = &pointer;
    return 0;
}
"#,
        ),
        (
            "assignment_record_pointer",
            "different record tags remain incompatible despite equal layouts",
            r#"struct Left { int value; };
struct Right { int value; };
int main(void) {
    struct Left *left = 0;
    struct Right *right = 0;
    left = right;
    return 0;
}
"#,
        ),
        (
            "assignment_signedness",
            "pointer assignment does not change the signedness of the pointed-to type",
            r#"int main(void) {
    unsigned int value = 0;
    int *pointer = 0;
    pointer = &value;
    return 0;
}
"#,
        ),
        (
            "assignment_function_type",
            "incompatible function parameter types cannot be assigned through pointers",
            r#"int main(void) {
    int (*left)(int) = 0;
    int (*right)(long) = 0;
    left = right;
    return 0;
}
"#,
        ),
        (
            "assignment_function_void",
            "a void pointer object does not implicitly convert to a function pointer",
            r#"int main(void) {
    void *opaque = 0;
    int (*function)(int) = 0;
    function = opaque;
    return 0;
}
"#,
        ),
        (
            "assignment_function_qualified_null",
            "a qualified void pointer zero is not a function null pointer constant",
            r#"int main(void) {
    int (*function)(int) = 0;
    function = (const void *)0;
    return 0;
}
"#,
        ),
        (
            "assignment_void_value",
            "a void expression cannot be assigned to an arithmetic object",
            r#"void nothing(void) {}
int main(void) {
    int value = 0;
    value = nothing();
    return 0;
}
"#,
        ),
        (
            "assignment_record_value",
            "simple record assignment requires the same canonical record type",
            r#"struct Left { int value; };
struct Right { int value; };
int main(void) {
    struct Left left = {0};
    struct Right right = {0};
    left = right;
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
