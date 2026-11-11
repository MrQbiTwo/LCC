// Arguments and returns cases for conversion constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "argument_arithmetic_decay",
            "fixed arguments convert arithmetic values and decay array and function expressions",
            r#"int twice(int value) {
    return value * 2;
}
int combine(int whole, double real, const int *values,
            int (*function)(int)) {
    return whole + (int)real + values[1] + function(3);
}
int main(void) {
    int values[2] = {2, 5};
    return combine(7.75, 11, values, twice) != 29;
}
"#,
        ),
        (
            "argument_void_pointer",
            "fixed arguments support qualifier-preserving object and void pointer conversion",
            r#"int read_value(const volatile int *pointer) {
    return *pointer;
}
int same(const volatile void *left, const volatile void *right) {
    return left == right;
}
int main(void) {
    int value = 61;
    const volatile void *opaque = &value;
    return read_value(opaque) != 61 || !same(&value, opaque);
}
"#,
        ),
        (
            "argument_null_constants",
            "fixed object and function pointer parameters accept integer and void null constants",
            r#"int empty(const int *object, int (*function)(int)) {
    return object == 0 && function == 0;
}
int main(void) {
    return !empty(2 - 2, (void *)0) || !empty((void *)0, 0);
}
"#,
        ),
        (
            "argument_record",
            "fixed record arguments accept qualified compatible values",
            r#"struct Pair { int left; int right; };
int sum(struct Pair pair) {
    return pair.left + pair.right;
}
int main(void) {
    const struct Pair pair = {31, 37};
    return sum(pair) != 68;
}
"#,
        ),
        (
            "argument_unspecified",
            "unspecified argument lists retain default arithmetic promotions",
            r#"int first();
int invoke(void) {
    char small = 71;
    float real = 2.5f;
    return first(small, real);
}
int first(int whole, double real) {
    return whole + (int)real;
}
int main(void) {
    return invoke() != 73;
}
"#,
        ),
        (
            "return_arithmetic",
            "return expressions convert to the declared arithmetic result type",
            r#"int whole(void) {
    return 79.75;
}
double real(void) {
    return 83;
}
long double extended(void) {
    return 89.5;
}
int main(void) {
    return !(whole() == 79 && real() == 83.0 && extended() == 89.5L);
}
"#,
        ),
        (
            "return_pointer",
            "pointer return conversion decays arrays and adds immediate qualifiers",
            r#"static int values[2] = {97, 101};
const int *view(void) {
    return values;
}
const volatile int *qualified(const int *pointer) {
    return pointer;
}
int main(void) {
    return view()[1] != 101 || *qualified(view()) != 97;
}
"#,
        ),
        (
            "return_void_pointer",
            "object and void pointer return conversions preserve the source qualifiers",
            r#"const void *opaque(const int *pointer) {
    return pointer;
}
const int *typed(const void *pointer) {
    return pointer;
}
int main(void) {
    int value = 103;
    return *typed(opaque(&value)) != 103;
}
"#,
        ),
        (
            "return_null_constants",
            "pointer results accept integer and unqualified void null constants",
            r#"typedef int (*Function)(int);
int *object(void) {
    return (int)0.75;
}
Function function(void) {
    return (void *)(3 - 3);
}
int main(void) {
    return object() != 0 || function() != 0;
}
"#,
        ),
        (
            "return_record",
            "record result conversion preserves compatible qualified record values",
            r#"struct Pair { int left; long right; };
struct Pair pair(void) {
    const struct Pair value = {107, 109};
    return value;
}
int main(void) {
    struct Pair value = pair();
    return value.left != 107 || value.right != 109;
}
"#,
        ),
        (
            "return_void_bare",
            "a void function may return without an expression",
            r#"static int value;
void update(void) {
    value = 113;
    return;
}
int main(void) {
    update();
    return value != 113;
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
