// Compatible pointers cases for conditional composite.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "array_unknown_first",
            "a known bound completes the conditional result when the unknown array comes first",
            r#"int main(void) {
    int values[3] = {3, 5, 7};
    int (*unknown)[] = &values;
    int (*known)[3] = &values;
    return sizeof(*(1 ? unknown : known)) != 3 * sizeof(int) ||
           (*(1 ? unknown : known))[2] != 7;
}
"#,
        ),
        (
            "array_known_first",
            "conditional result completion is independent of operand order and selected arm",
            r#"int main(void) {
    int values[4] = {11, 13, 17, 19};
    int (*unknown)[] = &values;
    int (*known)[4] = &values;
    return sizeof(*(0 ? known : unknown)) != 4 * sizeof(int) ||
           (*(0 ? known : unknown))[3] != 19;
}
"#,
        ),
        (
            "array_pointer_arithmetic",
            "the completed conditional pointee supplies the correct pointer arithmetic stride",
            r#"int main(void) {
    int rows[2][3] = {{2, 3, 5}, {7, 11, 13}};
    int (*unknown)[] = rows;
    int (*known)[3] = rows;
    return (1 ? unknown : known)[1][2] != 13 ||
           (0 ? known : unknown) + 2 - rows != 2;
}
"#,
        ),
        (
            "nested_pointers",
            "array bounds compose below multiple pointer layers",
            r#"int main(void) {
    int (**unknown)[] = 0;
    int (**known)[5] = 0;
    return sizeof(**(1 ? unknown : known)) != 5 * sizeof(int) ||
           sizeof(**(0 ? known : unknown)) != 5 * sizeof(int);
}
"#,
        ),
        (
            "array_pointer_elements",
            "composition reaches unknown array bounds within pointer elements of an outer array",
            r#"typedef int Unknown[];
typedef int Known[3];
int main(void) {
    Unknown *unknown[2] = {0, 0};
    Known *known[2] = {0, 0};
    return sizeof(***(1 ? &unknown : &known)) != 3 * sizeof(int) ||
           sizeof(*(0 ? &known : &unknown)) != 2 * sizeof(void *);
}
"#,
        ),
        (
            "nested_qualifiers",
            "matching nested qualifiers survive recursive composition while immediate qualifiers unite",
            r#"int main(void) {
    int (*const *left)[] = 0;
    int (*volatile *right)[3] = 0;
    return sizeof(**(1 ? left : right)) != 3 * sizeof(int);
}
"#,
        ),
        (
            "array_alias_unchanged",
            "constructing a conditional result does not complete the shared unknown-array typedef",
            r#"typedef int Unknown[];
typedef Unknown *Pointer;
int main(void) {
    Pointer first = 0;
    Pointer second = 0;
    int (*three)[3] = 0;
    int (*five)[5] = 0;
    return sizeof(*(1 ? first : three)) != 3 * sizeof(int) ||
           sizeof(*(1 ? second : five)) != 5 * sizeof(int) ||
           sizeof(*(0 ? five : first)) != 5 * sizeof(int);
}
"#,
        ),
        (
            "function_parameter_qualifiers",
            "function compatibility ignores only the top qualifiers of each parameter",
            r#"int read_value(int *pointer, int value) {
    return *pointer + value;
}
int main(void) {
    int value = 23;
    int (*plain)(int *, int) = read_value;
    int (*qualified)(int *const, const int) = read_value;
    return (1 ? plain : qualified)(&value, 5) != 28 ||
           (0 ? qualified : plain)(&value, 7) != 30;
}
"#,
        ),
        (
            "function_unspecified_first",
            "a learned prototype converts arguments when the unspecified function type comes first",
            r#"int combine(int whole, double real, const int *pointer) {
    return whole + (int)real + *pointer;
}
int main(void) {
    int value = 29;
    int (*unspecified)() = combine;
    int (*prototype)(int, double, const int *) = combine;
    return (1 ? unspecified : prototype)(3, 5, &value) != 37;
}
"#,
        ),
        (
            "function_prototype_first",
            "prototype composition preserves argument conversion with reversed operands",
            r#"int combine(int whole, double real, const int *pointer) {
    return whole + (int)real + *pointer;
}
int main(void) {
    int value = 31;
    int (*unspecified)() = combine;
    int (*prototype)(int, double, const int *) = combine;
    return (0 ? prototype : unspecified)(7, 11, &value) != 49;
}
"#,
        ),
        (
            "function_void_prototype",
            "an empty prototype composes with an unspecified parameter list",
            r#"int value(void) {
    return 41;
}
int main(void) {
    int (*unspecified)() = value;
    int (*prototype)(void) = value;
    return (1 ? unspecified : prototype)() != 41;
}
"#,
        ),
        (
            "function_enum_promotion",
            "promotion-stable signed enum parameters compose with unspecified function types",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 43 };
int identity(enum Value value) {
    return value;
}
int main(void) {
    int (*unspecified)() = identity;
    int (*prototype)(enum Value) = identity;
    return (1 ? unspecified : prototype)(POSITIVE) != 43;
}
"#,
        ),
        (
            "function_return_array",
            "function result pointers acquire the known array bound recursively",
            r#"typedef int Unknown[];
typedef int Known[3];
static Known values = {47, 53, 59};
Known *row(void) {
    return &values;
}
int main(void) {
    Unknown *(*unknown)(void) = row;
    Known *(*known)(void) = row;
    return sizeof(*(1 ? unknown : known)()) != 3 * sizeof(int) ||
           (*(0 ? known : unknown)())[2] != 59;
}
"#,
        ),
        (
            "function_return_nested_pointer",
            "function result composition reaches arrays beneath additional pointer layers",
            r#"typedef int Unknown[];
typedef int Known[4];
int main(void) {
    Unknown **(*unknown)(void) = 0;
    Known **(*known)(void) = 0;
    return sizeof(**(1 ? unknown : known)()) != 4 * sizeof(int);
}
"#,
        ),
        (
            "function_parameter_array",
            "function parameter pointees acquire known array bounds in the composite signature",
            r#"int read_value(int (*row)[3]) {
    return (*row)[2];
}
int main(void) {
    int values[3] = {61, 67, 71};
    int (*unknown)(int (*)[]) = read_value;
    int (*known)(int (*)[3]) = read_value;
    return (1 ? unknown : known)(&values) != 71 ||
           (0 ? known : unknown)(&values) != 71;
}
"#,
        ),
        (
            "function_parameters_from_both_arms",
            "a composite signature combines independent known bounds from both parameter lists",
            r#"int sum(int (*first)[3], long (*second)[2]) {
    return (*first)[2] + (int)(*second)[1];
}
int main(void) {
    int first[3] = {2, 3, 5};
    long second[2] = {7, 11};
    int (*left)(int (*)[], long (*)[2]) = sum;
    int (*right)(int (*)[3], long (*)[]) = sum;
    return (1 ? left : right)(&first, &second) != 16;
}
"#,
        ),
        (
            "function_source_signature_unchanged",
            "learning a prototype in a result leaves the source unspecified parameter list intact",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*one)(int) = 0;
    int (*two)(int, int) = 0;
    return sizeof((1 ? unspecified : one)(3)) != sizeof(int) ||
           sizeof((1 ? unspecified : two)(3, 5)) != sizeof(int) ||
           sizeof(unspecified()) != sizeof(int);
}
"#,
        ),
        (
            "function_parameter_graph_unchanged",
            "composing function parameter arrays leaves the original parameter type graph unchanged",
            r#"int main(void) {
    int three[3] = {2, 3, 5};
    int two[2] = {7, 11};
    int (*unknown)(int (*)[]) = 0;
    int (*known)(int (*)[3]) = 0;
    return sizeof((1 ? unknown : known)(&three)) != sizeof(int) ||
           sizeof(unknown(&two)) != sizeof(int);
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("conditional_composite_accept_{suffix}"),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }
}
