// Compatible types cases for type identity.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "enum_qualified_identity",
            "qualified copies and typedef aliases retain the same enum identity",
            r#"enum Color { NEGATIVE = -1, GREEN = 7 };
typedef enum Color Color;
int main(void) {
    Color values[2] = {NEGATIVE, GREEN};
    const Color *left = values;
    const volatile enum Color *right = values + 2;
    return !(left == values && right - left == 2 && left[1] == GREEN);
}
"#,
        ),
        (
            "enum_qualified_redeclaration",
            "repeated qualified enum declarations refer to the canonical enum",
            r#"enum Color { NEGATIVE = -1, GREEN = 7 };
extern const enum Color color;
extern const enum Color color;
const enum Color color = GREEN;
int main(void) {
    return color != GREEN;
}
"#,
        ),
        (
            "enum_anonymous_alias",
            "objects sharing one anonymous enum declaration have compatible types",
            r#"typedef enum { NEGATIVE = -1, VALUE = 11 } Value;
int main(void) {
    Value value = VALUE;
    Value *pointer = &value;
    const Value *view = pointer;
    return *view != 11;
}
"#,
        ),
        (
            "enum_parameter_qualifier",
            "top-level parameter qualifiers do not change the canonical enum parameter type",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 11 };
int identity(enum Value value);
int identity(const enum Value value) {
    return value;
}
int main(void) {
    return identity(POSITIVE) != 11;
}
"#,
        ),
        (
            "enum_conditional_qualifiers",
            "conditional pointer composition unions qualifiers without losing same-enum identity",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 13 };
int main(void) {
    enum Value values[2] = {NEGATIVE, POSITIVE};
    const enum Value *left = values;
    volatile enum Value *right = values + 1;
    const volatile enum Value *first = 1 ? left : right;
    const volatile enum Value *second = 0 ? left : right;
    return !(first == values && second == values + 1 &&
             *first == -1 && *second == 13);
}
"#,
        ),
        (
            "enum_scalar_conversions",
            "distinct enum identities still permit ordinary arithmetic value conversions",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 13 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 17 };
enum Second convert(enum First value) {
    return value;
}
int main(void) {
    enum First first = FIRST;
    enum Second second = first;
    first = SECOND;
    return !(first == 17 && second == 13 && convert(FIRST) == 13 &&
             first + second == 30);
}
"#,
        ),
        (
            "enum_int_compatibility",
            "an enum with a negative enumerator is compatible with the chosen signed int type",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 19 };
int main(void) {
    enum Value values[2] = {NEGATIVE, POSITIVE};
    enum Value *typed = values;
    int *integer = typed;
    const int *view = 1 ? typed : integer;
    typed = integer;
    return !(typed == integer && typed + 2 - integer == 2 && view[1] == 19);
}
"#,
        ),
        (
            "enum_int_declaration",
            "the enum's compatible integer type is accepted by object and array declarations",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 23 };
extern enum Value value;
extern int value;
enum Value value = POSITIVE;
extern enum Value values[2];
extern int values[2];
enum Value values[2] = {NEGATIVE, POSITIVE};
int main(void) {
    return value != 23 || values[0] != -1;
}
"#,
        ),
        (
            "enum_int_prototype",
            "compatible enum and integer types compose function results and parameters",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 29 };
enum Value identity(enum Value value);
int identity(int value) {
    return value;
}
int main(void) {
    enum Value (*typed)(enum Value) = identity;
    int (*integer)(int) = typed;
    return integer(POSITIVE) != 29 || typed != integer;
}
"#,
        ),
        (
            "enum_default_promotion",
            "enum parameters remain compatible with their default-promoted signed integer type",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 31 };
int identity(enum Value value) {
    return value;
}
int main(void) {
    int (*unspecified)() = identity;
    enum Value value = POSITIVE;
    return unspecified(value) != 31;
}
"#,
        ),
        (
            "enum_nested_pointer",
            "canonical enum identity survives qualifier copies within nested pointers",
            r#"enum Value { NEGATIVE = -1, POSITIVE = 37 };
int main(void) {
    const enum Value value = POSITIVE;
    const enum Value *pointer = &value;
    const enum Value **nested = &pointer;
    const enum Value *const *view = nested;
    return **view != 37 || view != nested;
}
"#,
        ),
        (
            "char_qualified_alias",
            "plain char typedef aliases and immediate qualifiers preserve type identity",
            r#"typedef char Character;
int main(void) {
    Character values[3] = {3, 5, 7};
    const char *begin = values;
    volatile Character *end = values + 3;
    return !(begin == values && end - begin == 3 && begin[2] == 7);
}
"#,
        ),
        (
            "char_scalar_conversions",
            "plain signed and unsigned char values retain ordinary arithmetic conversions",
            r#"signed char convert(char value) {
    return value;
}
int main(void) {
    char plain = 41;
    signed char signed_value = plain;
    unsigned char unsigned_value = signed_value;
    plain = unsigned_value;
    return !(plain == 41 && signed_value == 41 && unsigned_value == 41 &&
             convert(plain) == 41 && plain + signed_value == 82);
}
"#,
        ),
        (
            "char_string_arrays",
            "string literal initialization remains valid for all three character array types",
            r#"static char plain[] = "abc";
static signed char signed_text[] = "def";
static unsigned char unsigned_text[] = "ghi";
int main(void) {
    char local[] = "jk";
    signed char signed_local[] = "lm";
    unsigned char unsigned_local[] = "no";
    return !(plain[1] == 'b' && signed_text[2] == 'f' &&
             unsigned_text[0] == 'g' && local[1] == 'k' &&
             signed_local[0] == 'l' && unsigned_local[1] == 'o');
}
"#,
        ),
        (
            "char_redeclaration",
            "matching plain and explicitly signed character declarations remain valid",
            r#"extern char plain;
char plain = 43;
extern signed char signed_value;
signed char signed_value = 47;
char identity(char value);
char identity(char value) {
    return value;
}
int main(void) {
    return identity(plain) != 43 || signed_value != 47;
}
"#,
        ),
        (
            "char_explicit_pointer_cast",
            "explicit character pointer casts still permit character access to object bytes",
            r#"int main(void) {
    char value = 53;
    signed char *signed_view = (signed char *)&value;
    unsigned char *unsigned_view = (unsigned char *)&value;
    return *signed_view != 53 || *unsigned_view != 53;
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("type_identity_accept_{suffix}"),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }
}
