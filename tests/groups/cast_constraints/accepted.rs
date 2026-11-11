// Accepted cases for cast constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "integer_widths",
            "integer casts preserve signed values and defined unsigned narrowing",
            r#"int main(void) {
    long value = -23;
    unsigned char byte = (unsigned char)257;
    unsigned short half = (unsigned short)65537UL;
    return !((int)(short)value == -23 && byte == 1 && half == 1);
}
"#,
        ),
        (
            "unsigned_wrap",
            "negative integers convert modulo the unsigned destination width",
            r#"int main(void) {
    return !((unsigned int)-1 == 4294967295U &&
             (unsigned long)-1 == 18446744073709551615UL);
}
"#,
        ),
        (
            "floating_widths",
            "float double and binary80 casts preserve exactly representable values",
            r#"int main(void) {
    float f = (float)3.5L;
    double d = (double)f;
    long double wide = (long double)d;
    return !(f == 3.5f && d == 3.5 && wide == 3.5L);
}
"#,
        ),
        (
            "arithmetic_domains",
            "arithmetic casts truncate in-range fractions toward zero",
            r#"int main(void) {
    int negative = (int)-7.75L;
    unsigned int positive = (unsigned int)9.75;
    long double wide = (long double)-17;
    float small = (float)11U;
    return !(negative == -7 && positive == 9 && wide == -17.0L && small == 11.0f);
}
"#,
        ),
        (
            "enum_and_bitfields",
            "enum and bit-field operands remain arithmetic cast inputs",
            r#"enum Choice { LOW = 1, HIGH = 3 };
struct Bits { signed int low : 4; unsigned int high : 4; };
int main(void) {
    struct Bits bits = {-3, 11};
    enum Choice choice = (enum Choice)3;
    return !((double)bits.low == -3.0 && (long)bits.high == 11 && choice == HIGH);
}
"#,
        ),
        (
            "lp64_object_integer_roundtrip",
            "Linux x86_64 LP64 unsigned long retains an object pointer roundtrip",
            r#"int main(void) {
    int object = 19;
    unsigned long address = (unsigned long)&object;
    int *restored = (int *)address;
    return !(sizeof(unsigned long) == sizeof(int *) && restored == &object && *restored == 19);
}
"#,
        ),
        (
            "lp64_array_integer_roundtrip",
            "array decay precedes the LP64 pointer-to-integer cast",
            r#"int main(void) {
    int values[2] = {5, 23};
    unsigned long address = (unsigned long)values;
    int *restored = (int *)address;
    return !(restored == values && restored[1] == 23);
}
"#,
        ),
        (
            "null_integer_pointer",
            "integer zero casts produce null object and function pointers",
            r#"int main(void) {
    int *object = (int *)0;
    int (*function)(int) = (int (*)(int))0;
    return !(object == 0 && function == 0 && (void *)0 == 0);
}
"#,
        ),
        (
            "object_void_pointer",
            "object pointers roundtrip through void pointers",
            r#"int main(void) {
    int value = 29;
    void *opaque = (void *)&value;
    int *restored = (int *)opaque;
    return !(restored == &value && *restored == 29);
}
"#,
        ),
        (
            "qualified_pointer",
            "explicit pointer casts can add and remove pointee qualifiers",
            r#"int main(void) {
    int value = 31;
    const volatile int *qualified = (const volatile int *)&value;
    int *restored = (int *)qualified;
    *restored = 37;
    return value != 37;
}
"#,
        ),
        (
            "object_character_pointer",
            "an object pointer roundtrips through a character pointer",
            r#"int main(void) {
    long value = 41;
    char *bytes = (char *)&value;
    long *restored = (long *)bytes;
    return !(restored == &value && *restored == 41);
}
"#,
        ),
        (
            "function_designator",
            "function designators decay before a matching function-pointer cast",
            r#"int add_one(int value) {
    return value + 1;
}
int main(void) {
    return ((int (*)(int))add_one)(42) != 43;
}
"#,
        ),
        (
            "function_pointer_roundtrip",
            "different function-pointer types roundtrip before a compatible call",
            r#"int add_one(int value) {
    return value + 1;
}
int main(void) {
    int (*original)(int) = add_one;
    double (*other)(double) = (double (*)(double))original;
    int (*restored)(int) = (int (*)(int))other;
    return !(restored == original && restored(46) == 47);
}
"#,
        ),
        (
            "lp64_function_integer_roundtrip",
            "Linux x86_64 LP64 unsigned long retains a function pointer roundtrip",
            r#"int identity(int value) {
    return value;
}
int main(void) {
    unsigned long address = (unsigned long)identity;
    int (*restored)(int) = (int (*)(int))address;
    return !(restored == identity && restored(53) == 53);
}
"#,
        ),
        (
            "incomplete_record_pointer",
            "casts to pointers do not require completing their pointed-to record",
            r#"struct Hidden;
int main(void) {
    struct Hidden *hidden = (struct Hidden *)0;
    void *opaque = (void *)hidden;
    return !(hidden == 0 && opaque == 0);
}
"#,
        ),
        (
            "incomplete_array_pointer",
            "a cast to pointer-to-incomplete-array preserves later complete access",
            r#"int main(void) {
    int values[2] = {59, 61};
    int (*unknown)[] = (int (*)[])&values;
    int (*restored)[2] = (int (*)[2])unknown;
    return !(restored == &values && (*restored)[1] == 61);
}
"#,
        ),
        (
            "void_arithmetic_side_effect",
            "casting to void discards a scalar value while preserving side effects",
            r#"int main(void) {
    int count = 0;
    (void)(count += 3);
    (void)4.5L;
    return count != 3;
}
"#,
        ),
        (
            "void_array_function",
            "casting array and function designators to void is permitted",
            r#"int identity(int value) {
    return value;
}
int main(void) {
    int values[2] = {1, 2};
    (void)values;
    (void)identity;
    return 0;
}
"#,
        ),
        (
            "void_complete_records",
            "casting complete struct and union values to void is permitted",
            r#"struct Record { int value; };
union Number { int value; double real; };
int main(void) {
    struct Record record = {67};
    union Number number = {71};
    (void)record;
    (void)number;
    return !(record.value == 67 && number.value == 71);
}
"#,
        ),
        (
            "void_returned_record",
            "a discarded record-returning call still evaluates once",
            r#"struct Record { int value; };
int count;
struct Record make_record(void) {
    struct Record record = {73};
    count++;
    return record;
}
int main(void) {
    (void)make_record();
    return count != 1;
}
"#,
        ),
        (
            "void_operand",
            "a cast to void accepts an already-void operand",
            r#"int count;
void increment(void) {
    count++;
}
int main(void) {
    (void)increment();
    (void)(void)(count++);
    return count != 2;
}
"#,
        ),
        (
            "void_conditional_operand",
            "a void conditional can be cast to void without evaluating both arms",
            r#"int main(void) {
    int count = 0;
    (void)(1 ? (void)(count += 2) : (void)(count += 10));
    return count != 2;
}
"#,
        ),
        (
            "sizeof_cast_unevaluated",
            "sizeof a valid cast checks its type without evaluating its operand",
            r#"int main(void) {
    int count = 0;
    unsigned long size = sizeof((long)(count++));
    return !(size == 8 && count == 0);
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("cast_{suffix}"),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }
}
