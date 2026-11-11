// Distinct characters cases for type identity.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "char_plain_signed_equality",
            "plain char and signed char are distinct pointee types",
            r#"int main(void) {
    char *plain = 0;
    signed char *explicit_signed = 0;
    return sizeof(plain == explicit_signed);
}
"#,
        ),
        (
            "char_plain_unsigned_equality",
            "plain char and unsigned char are distinct pointee types",
            r#"int main(void) {
    char *plain = 0;
    unsigned char *explicit_unsigned = 0;
    return sizeof(plain == explicit_unsigned);
}
"#,
        ),
        (
            "char_signed_unsigned_equality",
            "signed char and unsigned char remain distinct pointee types",
            r#"int main(void) {
    signed char *explicit_signed = 0;
    unsigned char *explicit_unsigned = 0;
    return sizeof(explicit_signed == explicit_unsigned);
}
"#,
        ),
        (
            "char_relational",
            "equal character representation does not permit mixed pointer ordering",
            r#"int main(void) {
    char *plain = 0;
    signed char *explicit_signed = 0;
    return sizeof(plain < explicit_signed);
}
"#,
        ),
        (
            "char_difference",
            "equal character sizes do not permit mixed pointer subtraction",
            r#"int main(void) {
    char *plain = 0;
    signed char *explicit_signed = 0;
    return sizeof(plain - explicit_signed);
}
"#,
        ),
        (
            "char_assignment",
            "pointer assignment does not implicitly change plain char into signed char",
            r#"int main(void) {
    char *plain = 0;
    signed char *explicit_signed = 0;
    plain = explicit_signed;
    return 0;
}
"#,
        ),
        (
            "char_initializer",
            "pointer initialization does not implicitly change signed char into plain char",
            r#"int main(void) {
    char value = 3;
    signed char *pointer = &value;
    return 0;
}
"#,
        ),
        (
            "char_string_pointer_initializer",
            "a string literal decays to plain char pointer rather than signed char pointer",
            r#"int main(void) {
    signed char *text = "abc";
    return 0;
}
"#,
        ),
        (
            "char_argument",
            "fixed pointer arguments distinguish plain and explicitly signed char",
            r#"int read_value(const signed char *pointer) {
    return *pointer;
}
int main(void) {
    char value = 3;
    return sizeof(read_value(&value));
}
"#,
        ),
        (
            "char_return",
            "pointer return conversion distinguishes plain and explicitly signed char",
            r#"signed char *convert(char *pointer) {
    return pointer;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "char_conditional",
            "conditional pointer arms distinguish plain and explicitly signed char",
            r#"int main(void) {
    char *plain = 0;
    signed char *explicit_signed = 0;
    return sizeof(1 ? plain : explicit_signed);
}
"#,
        ),
        (
            "char_qualified_distinct",
            "ignoring immediate pointer qualifiers must preserve plain char identity",
            r#"int main(void) {
    const char *plain = 0;
    volatile signed char *explicit_signed = 0;
    return sizeof(plain == explicit_signed);
}
"#,
        ),
        (
            "char_nested_pointer",
            "nested pointer compatibility retains all three character identities",
            r#"int main(void) {
    char **plain = 0;
    signed char **explicit_signed = 0;
    return sizeof(plain == explicit_signed);
}
"#,
        ),
        (
            "char_object_redeclaration",
            "object redeclarations cannot replace plain char with signed char",
            r#"extern char value;
extern signed char value;
int main(void) {
    return 0;
}
"#,
        ),
        (
            "char_array_redeclaration",
            "array declaration composition distinguishes plain and signed character elements",
            r#"extern char values[2];
extern signed char values[2];
int main(void) {
    return 0;
}
"#,
        ),
        (
            "char_function_result",
            "function redeclarations distinguish plain and signed character results",
            r#"char value(void);
signed char value(void);
int main(void) {
    return 0;
}
"#,
        ),
        (
            "char_function_parameter",
            "prototype redeclarations distinguish plain and signed character parameters",
            r#"int value(char parameter);
int value(signed char parameter);
int main(void) {
    return 0;
}
"#,
        ),
        (
            "char_function_pointer",
            "function pointer compatibility distinguishes plain and signed character parameters",
            r#"int main(void) {
    int (*plain)(char) = 0;
    int (*explicit_signed)(signed char) = 0;
    return sizeof(plain == explicit_signed);
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("type_identity_reject_{suffix}"),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
