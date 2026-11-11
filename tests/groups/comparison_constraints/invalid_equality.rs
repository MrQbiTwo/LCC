// Invalid equality cases for comparison constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, declarations, expression) in [
        (
            "pointer_nonzero",
            "nonzero integers are not null pointer constants",
            r#"int *pointer;"#,
            "pointer == 1",
        ),
        (
            "nonzero_pointer",
            "reversed operands still require an integer null constant",
            r#"int *pointer;"#,
            "1 != pointer",
        ),
        (
            "pointer_variable_zero",
            "a zero-valued object is not an integer constant expression",
            r#"int *pointer;
int zero;"#,
            "pointer == zero",
        ),
        (
            "pointer_const_zero",
            "a const object is not an integer constant expression in C",
            r#"int *pointer;
const int zero = 0;"#,
            "zero != pointer",
        ),
        (
            "pointer_float_zero",
            "floating zero is not a null pointer constant",
            r#"int *pointer;"#,
            "pointer == 0.0",
        ),
        (
            "float_pointer",
            "arithmetic equality cannot combine floating and pointer operands",
            r#"int *pointer;"#,
            "1.0f != pointer",
        ),
        (
            "pointer_comma_zero",
            "an evaluated comma expression is not a null pointer constant",
            r#"int *pointer;"#,
            "pointer == (1, 0)",
        ),
        (
            "pointer_assignment_zero",
            "an assignment expression is not a null pointer constant",
            r#"int *pointer;
int zero;"#,
            "pointer == (zero = 0)",
        ),
        (
            "pointer_call_zero",
            "a function call is not a null pointer constant",
            r#"int *pointer;
int zero(void) { return 0; }"#,
            "pointer == zero()",
        ),
        (
            "pointer_folded_zero",
            "constant folding does not turn an object expression into a null constant",
            r#"int *pointer;
int value;"#,
            "pointer == (value - value)",
        ),
        (
            "void_equality",
            "void expressions are not equality operands",
            r#"void nothing(void) {}"#,
            "nothing() == nothing()",
        ),
        (
            "record_equality",
            "record values are not equality operands",
            r#"struct Record { int value; };
struct Record object;"#,
            "object == object",
        ),
        (
            "union_equality",
            "union values are not equality operands",
            r#"union Value { int value; };
union Value object;"#,
            "object != object",
        ),
        (
            "record_identity",
            "matching record layouts do not establish compatible types",
            r#"struct Left { int value; };
struct Right { int value; };
struct Left *left;
struct Right *right;"#,
            "left == right",
        ),
        (
            "integer_signedness",
            "signed and unsigned integer pointees are distinct types",
            r#"int *left;
unsigned int *right;"#,
            "left != right",
        ),
        (
            "nested_qualifier",
            "nested pointee qualifiers remain part of compatibility",
            r#"int **left;
const int **right;"#,
            "left == right",
        ),
        (
            "array_bound",
            "different known array bounds are incompatible",
            r#"int (*left)[2];
int (*right)[3];"#,
            "left != right",
        ),
        (
            "function_return",
            "function result types must be compatible",
            r#"int (*left)(int);
long (*right)(int);"#,
            "left == right",
        ),
        (
            "function_parameter",
            "function parameter types must be compatible",
            r#"int (*left)(int);
int (*right)(long);"#,
            "left != right",
        ),
        (
            "function_arity",
            "prototype parameter counts must agree",
            r#"int (*left)(int);
int (*right)(int, int);"#,
            "left == right",
        ),
        (
            "function_variadic",
            "variadic and fixed function prototypes are incompatible",
            r#"int (*left)(int);
int (*right)(int, ...);"#,
            "left != right",
        ),
        (
            "function_nested_qualifier",
            "qualifiers within a pointer parameter remain significant",
            r#"int (*left)(int *);
int (*right)(const int *);"#,
            "left == right",
        ),
        (
            "old_style_float",
            "float parameters change under default argument promotions",
            r#"int (*left)();
int (*right)(float);"#,
            "left == right",
        ),
        (
            "old_style_char",
            "character parameters change under default argument promotions",
            r#"int (*left)();
int (*right)(char);"#,
            "left != right",
        ),
        (
            "old_style_variadic",
            "unspecified parameters cannot match a variadic prototype",
            r#"int (*left)();
int (*right)(int, ...);"#,
            "left == right",
        ),
        (
            "function_object",
            "function and object pointers are incompatible equality operands",
            r#"int (*function)(int);
int *object;"#,
            "function == object",
        ),
        (
            "function_void_object",
            "a void pointer object is not a null pointer constant",
            r#"int (*function)(int);
void *opaque;"#,
            "function == opaque",
        ),
        (
            "function_void_nonzero",
            "a nonzero void-pointer cast is not a null pointer constant",
            r#"int (*function)(int);"#,
            "function != (void *)1",
        ),
        (
            "function_qualified_void_zero",
            "qualified void-pointer casts are not null pointer constants",
            r#"int (*function)(int);"#,
            "function == (const void *)0",
        ),
    ] {
        cases.push(Case::new(
            format!("comparison_reject_{suffix}"),
            purpose,
            format!("{declarations}\nint main(void) {{\n    return sizeof({expression});\n}}\n"),
            Expectation::Reject("E008".into()),
        ));
    }
}
