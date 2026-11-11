// Constraints cases for cast constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Each row changes the cast constraint being violated. The common body is
    // deliberately small; rejection is E008, independent of diagnostic wording.
    for (suffix, purpose, declarations, expression) in [
        (
            "void_to_integer",
            "void values cannot be converted to integers",
            "void nothing(void) {}",
            "(int)nothing()",
        ),
        (
            "void_to_floating",
            "void values cannot be converted to floating types",
            "void nothing(void) {}",
            "(double)nothing()",
        ),
        (
            "void_to_pointer",
            "void values cannot be converted to pointers",
            "void nothing(void) {}",
            "(int *)nothing()",
        ),
        (
            "void_conditional_to_integer",
            "void conditional results cannot be converted to integers",
            "",
            "(int)(1 ? (void)0 : (void)1)",
        ),
        (
            "struct_to_integer",
            "struct values are not scalar cast operands",
            r#"struct Record { int value; };
struct Record value;"#,
            "(int)value",
        ),
        (
            "struct_to_floating",
            "struct values cannot be converted to floating types",
            r#"struct Record { int value; };
struct Record value;"#,
            "(double)value",
        ),
        (
            "struct_to_pointer",
            "struct values cannot be converted to pointers",
            r#"struct Record { int value; };
struct Record value;"#,
            "(int *)value",
        ),
        (
            "union_to_integer",
            "union values are not scalar cast operands",
            r#"union Number { int value; double real; };
union Number number;"#,
            "(int)number",
        ),
        (
            "returned_record_to_integer",
            "record-returning calls remain nonscalar operands",
            r#"struct Record { int value; };
struct Record make_record(void) {
    struct Record value = {1};
    return value;
}"#,
            "(int)make_record()",
        ),
        (
            "target_struct",
            "a struct cannot be the destination of a scalar cast",
            "struct Record { int value; };",
            "(struct Record)1",
        ),
        (
            "target_same_struct",
            "casting a struct to its own type is still invalid C",
            r#"struct Record { int value; };
struct Record value;"#,
            "(struct Record)value",
        ),
        (
            "target_union",
            "a union cannot be a cast destination even for the same union",
            r#"union Number { int value; double real; };
union Number number;"#,
            "(union Number)number",
        ),
        (
            "target_incomplete_record",
            "an incomplete record is not a scalar cast destination",
            "struct Hidden;",
            "(struct Hidden)0",
        ),
        (
            "target_array",
            "an array type cannot be a cast destination",
            "int values[2];",
            "(int [2])values",
        ),
        (
            "target_function",
            "a function type cannot be a cast destination",
            "int identity(int value) { return value; }",
            "(int (int))identity",
        ),
        (
            "float_to_pointer",
            "float values cannot be cast directly to pointers",
            "",
            "(int *)1.0f",
        ),
        (
            "double_to_pointer",
            "double values cannot be cast directly to pointers",
            "",
            "(int *)1.0",
        ),
        (
            "binary80_to_pointer",
            "binary80 values cannot be cast directly to pointers",
            "",
            "(int *)1.0L",
        ),
        (
            "floating_to_function_pointer",
            "floating values cannot be cast to function pointers",
            "",
            "(int (*)(int))1.0",
        ),
        (
            "floating_to_incomplete_pointer",
            "an incomplete pointee does not permit floating-to-pointer casts",
            "struct Hidden;",
            "(struct Hidden *)1.0",
        ),
        (
            "pointer_to_float",
            "object pointers cannot be cast directly to float",
            "int value;",
            "(float)&value",
        ),
        (
            "pointer_to_double",
            "object pointers cannot be cast directly to double",
            "int value;",
            "(double)&value",
        ),
        (
            "pointer_to_binary80",
            "object pointers cannot be cast directly to binary80",
            "int value;",
            "(long double)&value",
        ),
        (
            "function_pointer_to_floating",
            "function pointers cannot be cast directly to floating types",
            "int identity(int value) { return value; }",
            "(double)&identity",
        ),
        (
            "array_to_floating",
            "array decay does not permit a floating cast destination",
            "int values[2];",
            "(double)values",
        ),
        (
            "function_to_floating",
            "function decay does not permit a floating cast destination",
            "int identity(int value) { return value; }",
            "(float)identity",
        ),
        (
            "sizeof_invalid_pointer_cast",
            "unevaluated sizeof operands still obey cast constraints",
            "",
            "sizeof((int *)1.0)",
        ),
        (
            "sizeof_invalid_array_target",
            "sizeof does not make an array cast destination valid",
            "int values[2];",
            "sizeof((int [2])values)",
        ),
        (
            "void_outer_invalid_inner",
            "discarding an invalid inner cast does not make it valid",
            "void nothing(void) {}",
            "(void)(int)nothing()",
        ),
        (
            "void_outer_record_target",
            "discarding a record cast does not make the cast valid",
            r#"struct Record { int value; };
struct Record value;"#,
            "(void)(struct Record)value",
        ),
    ] {
        cases.push(Case::new(
            format!("cast_reject_{suffix}"),
            purpose,
            format!("{declarations}\nint main(void) {{\n    {expression};\n    return 0;\n}}\n"),
            Expectation::Reject("E008".into()),
        ));
    }
}
