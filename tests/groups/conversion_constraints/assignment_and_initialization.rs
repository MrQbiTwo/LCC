// Assignment and initialization cases for conversion constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "assignment_arithmetic",
            "simple assignment converts between supported arithmetic types",
            r#"int main(void) {
    int integer = 0;
    unsigned char byte = 0;
    float single = 0;
    double real = 0;
    long double extended = 0;
    integer = 7.75;
    byte = 259;
    single = integer;
    real = single;
    extended = real;
    return !(integer == 7 && byte == 3 && extended == 7.0L);
}
"#,
        ),
        (
            "assignment_record",
            "qualified record values retain canonical identity during assignment",
            r#"struct Pair { int left; long right; };
int main(void) {
    const struct Pair original = {11, 13};
    struct Pair copy = {0, 0};
    copy = original;
    return !(copy.left == 11 && copy.right == 13);
}
"#,
        ),
        (
            "assignment_qualifiers",
            "pointer assignment may add immediate pointee qualifiers",
            r#"int main(void) {
    int value = 17;
    int *plain = &value;
    const int *read_only = 0;
    const volatile int *both = 0;
    int *const *nested = 0;
    read_only = plain;
    both = read_only;
    nested = &plain;
    return !(*read_only == 17 && *both == 17 && **nested == 17);
}
"#,
        ),
        (
            "assignment_void_roundtrip",
            "object and void pointers convert in both directions without losing qualifiers",
            r#"int main(void) {
    const volatile int value = 19;
    const volatile void *opaque = &value;
    const volatile int *typed = 0;
    typed = opaque;
    opaque = typed;
    return !(typed == &value && opaque == &value && *typed == 19);
}
"#,
        ),
        (
            "assignment_incomplete",
            "void conversion accepts pointers to incomplete object types",
            r#"struct Hidden;
int main(void) {
    struct Hidden *hidden = 0;
    void *opaque = 0;
    opaque = hidden;
    hidden = opaque;
    return hidden != 0;
}
"#,
        ),
        (
            "assignment_array_bounds",
            "compatible array pointer types may combine known and unknown bounds",
            r#"int main(void) {
    int values[3] = {23, 29, 31};
    int (*unknown)[] = 0;
    int (*known)[3] = 0;
    unknown = &values;
    known = unknown;
    return (*known)[2] != 31;
}
"#,
        ),
        (
            "assignment_null_constants",
            "integer constant expression zero remains a null pointer constant",
            r#"enum Zero { ZERO = 0 };
int main(void) {
    int *pointer = 0;
    pointer = 3 - 3;
    pointer = ZERO;
    pointer = sizeof(int) - sizeof(int);
    pointer = (int)0.75;
    return pointer != 0;
}
"#,
        ),
        (
            "assignment_function",
            "function decay and promotion-stable unspecified prototypes are compatible",
            r#"int identity(int value) {
    return value;
}
int main(void) {
    int (*function)(const int) = 0;
    int (*unspecified)() = 0;
    function = identity;
    unspecified = function;
    function = unspecified;
    return function(37) != 37;
}
"#,
        ),
        (
            "assignment_function_null",
            "unqualified void-pointer casts of integer zero initialize function pointers",
            r#"int main(void) {
    int (*function)(int) = (void *)0;
    function = (void *)(2 - 2);
    return function != 0;
}
"#,
        ),
        (
            "assignment_explicit_cast",
            "explicit pointer and integer conversions remain an available target operation",
            r#"int main(void) {
    int value = 41;
    const int *read_only = &value;
    unsigned long address = 0;
    int *pointer = 0;
    address = (unsigned long)read_only;
    pointer = (int *)address;
    pointer = (int *)read_only;
    return pointer != &value || *pointer != 41;
}
"#,
        ),
        (
            "initializer_arithmetic",
            "scalar and aggregate element initializers perform arithmetic conversions",
            r#"struct Values { int whole; double real; };
static int global = 7.75;
static double real = 9;
int main(void) {
    int local = {11.75};
    unsigned char bytes[2] = {257, 258};
    struct Values values = {13.75, 17};
    return !(global == 7 && real == 9.0 && local == 11 &&
             bytes[0] == 1 && bytes[1] == 2 &&
             values.whole == 13 && values.real == 17.0);
}
"#,
        ),
        (
            "initializer_pointer_members",
            "array and structure pointer initializers decay arrays and preserve qualifiers",
            r#"struct View { const int *begin; const void *opaque; };
static int values[2] = {43, 47};
static const int *pointers[2] = {values, values + 1};
static struct View view = {values, values};
int main(void) {
    return !(pointers[1][0] == 47 && view.begin[0] == 43 &&
             view.opaque == values);
}
"#,
        ),
        (
            "initializer_record_copy",
            "automatic record and union copies use compatible record identity",
            r#"struct Record { int value; };
union Value { int whole; double real; };
int main(void) {
    const struct Record original = {53};
    struct Record copy = original;
    const union Value source = {59};
    union Value result = source;
    return copy.value != 53 || result.whole != 59;
}
"#,
        ),
        (
            "initializer_nested_null",
            "pointer elements and members accept both C89 null pointer constant forms",
            r#"struct Links { int *object; int (*function)(int); };
static struct Links links = {(void *)0, (void *)(3 - 3)};
static int *pointers[2] = {0, (int)0.5};
int main(void) {
    return links.object != 0 || links.function != 0 ||
           pointers[0] != 0 || pointers[1] != 0;
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("conversion_accept_{suffix}"),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }
}
