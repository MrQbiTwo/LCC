// Distinct enums cases for type identity.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Negative enumerators make both foreign compilers choose signed int;
    // distinct enum tags must still remain distinct from each other.
    for (suffix, purpose, body) in [
        (
            "enum_equality",
            "different enum tags do not have compatible pointees for equality",
            "enum First *left = 0;\n    enum Second *right = 0;\n    return sizeof(left == right);",
        ),
        (
            "enum_relational",
            "different enum tags do not have compatible pointees for relational comparisons",
            "enum First *left = 0;\n    enum Second *right = 0;\n    return sizeof(left < right);",
        ),
        (
            "enum_difference",
            "equal enum representations do not establish compatible pointer subtraction types",
            "enum First *left = 0;\n    enum Second *right = 0;\n    return sizeof(left - right);",
        ),
        (
            "enum_assignment",
            "pointer assignment preserves distinct enum tag identities",
            "enum First *left = 0;\n    enum Second *right = 0;\n    left = right;\n    return 0;",
        ),
        (
            "enum_initializer",
            "pointer initialization preserves distinct enum tag identities",
            "enum First value = FIRST;\n    enum Second *pointer = &value;\n    return 0;",
        ),
        (
            "enum_conditional",
            "conditional pointer arms require compatible enum identities",
            "enum First *left = 0;\n    enum Second *right = 0;\n    return sizeof(1 ? left : right);",
        ),
        (
            "enum_qualified_distinct",
            "ignoring immediate qualifiers must not erase the canonical enum identity",
            "const enum First *left = 0;\n    volatile enum Second *right = 0;\n    return sizeof(left == right);",
        ),
        (
            "enum_nested_pointer",
            "nested pointer compatibility retains enum identity",
            "enum First **left = 0;\n    enum Second **right = 0;\n    return sizeof(left == right);",
        ),
        (
            "enum_array_pointer",
            "array element compatibility retains enum identity beneath pointers",
            "enum First (*left)[2] = 0;\n    enum Second (*right)[2] = 0;\n    return sizeof(left == right);",
        ),
        (
            "enum_function_pointer",
            "function parameter compatibility retains enum identity",
            "int (*left)(enum First) = 0;\n    int (*right)(enum Second) = 0;\n    return sizeof(left == right);",
        ),
    ] {
        cases.push(Case::new(
            format!("type_identity_reject_{suffix}"),
            purpose,
            format!(
                "enum First {{ FIRST_NEGATIVE = -1, FIRST = 3 }};\n\
                 enum Second {{ SECOND_NEGATIVE = -1, SECOND = 5 }};\n\
                 int main(void) {{\n    {body}\n}}\n"
            ),
            Expectation::Reject("E008".into()),
        ));
    }

    for (suffix, purpose, source) in [
        (
            "enum_argument",
            "fixed pointer arguments retain enum tag identity",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 3 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 5 };
int read_value(const enum First *pointer) {
    return *pointer;
}
int main(void) {
    enum Second value = SECOND;
    return sizeof(read_value(&value));
}
"#,
        ),
        (
            "enum_return",
            "pointer return conversion retains enum tag identity",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 3 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 5 };
enum First *convert(enum Second *pointer) {
    return pointer;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "enum_anonymous",
            "separate anonymous enum declarations each create a distinct type",
            r#"int main(void) {
    enum { FIRST_NEGATIVE = -1, FIRST = 3 } first = FIRST;
    enum { SECOND_NEGATIVE = -1, SECOND = 5 } second = SECOND;
    return sizeof(&first == &second);
}
"#,
        ),
        (
            "enum_shadow",
            "an inner enum definition creates a distinct identity despite the same tag spelling",
            r#"enum Value { OUTER_NEGATIVE = -1, OUTER = 3 };
int main(void) {
    enum Value outer = OUTER;
    {
        enum Value { INNER_NEGATIVE = -1, INNER = 5 };
        enum Value inner = INNER;
        return sizeof(&outer == &inner);
    }
}
"#,
        ),
        (
            "enum_object_redeclaration",
            "object declarations cannot replace one enum tag with another",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 3 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 5 };
extern enum First value;
extern enum Second value;
int main(void) {
    return 0;
}
"#,
        ),
        (
            "enum_array_redeclaration",
            "array declaration composition retains enum element identity",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 3 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 5 };
extern enum First values[2];
extern enum Second values[2];
int main(void) {
    return 0;
}
"#,
        ),
        (
            "enum_function_result",
            "function redeclarations cannot replace an enum result with a distinct enum",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 3 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 5 };
enum First value(void);
enum Second value(void);
int main(void) {
    return 0;
}
"#,
        ),
        (
            "enum_function_parameter",
            "prototype redeclarations retain enum parameter identity",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 3 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 5 };
int value(enum First parameter);
int value(enum Second parameter);
int main(void) {
    return 0;
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
