// Compatible values cases for comparison constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "arithmetic",
            "comparison operands use the usual integer and floating conversions",
            r#"int main(void) {
    return !(-1 > 1U && -1L < 1U && 3 == 3.0 &&
             3.25f < 3.5L && 4.0L >= 4 && 5.0 != 6);
}
"#,
        ),
        (
            "array_order",
            "array pointers compare in element order including one-past pointers",
            r#"int main(void) {
    int values[3] = {2, 3, 5};
    return !(values == &values[0] && values != values + 1 &&
             values < values + 3 && values + 3 > values &&
             values <= values && values + 1 >= values);
}
"#,
        ),
        (
            "qualified_object",
            "immediate pointee qualifiers do not prevent pointer comparisons",
            r#"int main(void) {
    int values[2] = {7, 11};
    const int *left = values;
    volatile int *right = values + 1;
    return !(left == values && left != right && left < right && right >= left);
}
"#,
        ),
        (
            "nested_pointer",
            "matching nested pointer types remain comparable",
            r#"int main(void) {
    int *pointers[2] = {0, 0};
    int *const *qualified = pointers;
    return !(qualified == pointers && qualified < pointers + 2);
}
"#,
        ),
        (
            "incomplete_record",
            "compatible incomplete record pointers allow equality and relational operands",
            r#"struct Hidden;
int main(void) {
    struct Hidden *left = 0;
    const struct Hidden *right = 0;
    return !(left == right && sizeof(left < right) == sizeof(int));
}
"#,
        ),
        (
            "unknown_array_bound",
            "known and unknown array bounds remain compatible through pointer types",
            r#"int main(void) {
    int values[3] = {17, 19, 23};
    int (*unknown)[] = &values;
    int (*known)[3] = &values;
    return unknown != known;
}
"#,
        ),
        (
            "nested_unknown_array_bound",
            "array compatibility applies recursively below another pointer",
            r#"int main(void) {
    int (**unknown)[] = 0;
    int (**known)[3] = 0;
    return !(unknown == known && sizeof(unknown < known) == sizeof(int));
}
"#,
        ),
        (
            "function_decay",
            "function designators decay for equality with function pointers",
            r#"int identity(int value) {
    return value;
}
int main(void) {
    int (*pointer)(int) = identity;
    return !(identity == pointer && pointer == identity && identity != 0);
}
"#,
        ),
        (
            "incomplete_array_relational",
            "compatible incomplete array pointees satisfy relational operand constraints",
            r#"int main(void) {
    int (*left)[] = 0;
    int (*right)[] = 0;
    return sizeof(left < right) != sizeof(int);
}
"#,
        ),
        (
            "function_designators",
            "two bare function designators both decay before equality comparison",
            r#"int first(int value) {
    return value;
}
int second(int value) {
    return value + 1;
}
int main(void) {
    return !(first == first && first != second);
}
"#,
        ),
        (
            "function_parameter_qualifier",
            "function compatibility ignores outer parameter qualifiers",
            r#"int identity(int value) {
    return value;
}
int main(void) {
    int (*plain)(int) = identity;
    int (*qualified)(const int) = identity;
    return plain != qualified;
}
"#,
        ),
        (
            "function_pointer_parameter_qualifier",
            "outer pointer parameter qualifiers do not change a function type",
            r#"int read_value(int *pointer) {
    return *pointer;
}
int main(void) {
    int (*plain)(int *) = read_value;
    int (*qualified)(int *const) = read_value;
    return plain != qualified;
}
"#,
        ),
        (
            "old_style_integer_prototype",
            "an unspecified parameter list is compatible with promotion-stable integer parameters",
            r#"int identity(int value) {
    return value;
}
int main(void) {
    int (*unspecified)() = identity;
    int (*prototype)(int) = identity;
    return unspecified != prototype;
}
"#,
        ),
        (
            "old_style_double_prototype",
            "double parameters are stable under default argument promotions",
            r#"int whole(double value) {
    return (int)value;
}
int main(void) {
    int (*unspecified)() = whole;
    int (*prototype)(double) = whole;
    return unspecified != prototype;
}
"#,
        ),
        (
            "old_style_pointer_prototype",
            "pointer parameters are stable under default argument promotions",
            r#"struct Hidden;
int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(struct Hidden *) = 0;
    return unspecified != prototype;
}
"#,
        ),
        (
            "old_style_void_prototype",
            "an empty prototype is compatible with an unspecified parameter list",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(void) = 0;
    return unspecified != prototype;
}
"#,
        ),
        (
            "void_object_equality",
            "qualified void pointers compare equally with the original object pointer",
            r#"int main(void) {
    int value = 29;
    const void *opaque = &value;
    return !(opaque == &value && &value == opaque);
}
"#,
        ),
        (
            "void_incomplete_equality",
            "void pointers can be compared with incomplete object pointers",
            r#"struct Hidden;
int main(void) {
    struct Hidden *hidden = 0;
    const void *opaque = 0;
    return hidden != opaque;
}
"#,
        ),
        (
            "void_relational",
            "compatible void pointees satisfy the incomplete-type relational constraint",
            r#"int main(void) {
    void *left = 0;
    const volatile void *right = 0;
    return sizeof(left < right) != sizeof(int);
}
"#,
        ),
        (
            "null_integer_constants",
            "integer constant expressions equal to zero compare with object pointers",
            r#"enum Zero { ZERO = 0 };
int main(void) {
    int *pointer = 0;
    return !(pointer == 0 && 0 == pointer && pointer == (3 - 3) &&
             pointer == ZERO && pointer == (int)0.75 &&
             pointer == (sizeof(int) - sizeof(int)));
}
"#,
        ),
        (
            "null_void_cast",
            "unqualified void-pointer casts of integer zero are null pointer constants",
            r#"int main(void) {
    int *object = 0;
    int (*function)(int) = 0;
    return !(object == (void *)0 && (void *)(2 - 2) == function &&
             function == (void *)0);
}
"#,
        ),
        (
            "function_null_integer",
            "integer null constants compare with function pointers in either order",
            r#"int main(void) {
    int (*function)(int) = 0;
    return !(function == 0 && 0 == function && function == (1 - 1));
}
"#,
        ),
        (
            "pointer_difference",
            "compatible pointer subtraction counts elements and supports negative differences",
            r#"int main(void) {
    int values[4] = {2, 3, 5, 7};
    return !(values + 4 - values == 4 && values - (values + 3) == -3);
}
"#,
        ),
        (
            "qualified_difference",
            "immediate pointee qualifiers do not change pointer subtraction compatibility",
            r#"int main(void) {
    long values[3] = {11, 13, 17};
    const long *left = values;
    volatile long *right = values + 3;
    return !(right - left == 3 && left - right == -3);
}
"#,
        ),
        (
            "array_row_difference",
            "pointer subtraction uses the size of a complete array pointee",
            r#"int main(void) {
    int rows[3][2] = {{1, 2}, {3, 4}, {5, 6}};
    return rows + 3 - rows != 3;
}
"#,
        ),
        (
            "record_difference",
            "qualified views retain canonical record identity for pointer differences",
            r#"struct Record { int value; };
int main(void) {
    struct Record records[3] = {{19}, {23}, {29}};
    const struct Record *left = records;
    return records + 3 - left != 3;
}
"#,
        ),
        (
            "nested_pointer_difference",
            "pointer array elements remain complete objects despite incomplete nested pointees",
            r#"struct Hidden;
int main(void) {
    struct Hidden *pointers[2] = {0, 0};
    struct Hidden *const *left = pointers;
    return pointers + 2 - left != 2;
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("comparison_accept_{suffix}"),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }
}
