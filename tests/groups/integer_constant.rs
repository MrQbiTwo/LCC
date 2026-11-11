// Independent expected values cover target conversions in constants and code.
use crate::{Case, Expectation};

// Exercise the same expression at static initialization and runtime boundaries.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (name, expression, expected) in [
        ("equal_uint", "-1 == 0xffffffffU", "1"),
        ("unequal_uint", "-1 != 0xffffffffU", "0"),
        ("less_uint", "-1 < 0xffffffffU", "0"),
        ("less_equal_uint", "-1 <= 0xffffffffU", "1"),
        ("greater_uint", "-1 > 0xffffffffU", "0"),
        ("greater_equal_uint", "-1 >= 0xffffffffU", "1"),
        ("reverse_less", "0xfffffffeU < -1", "1"),
        ("reverse_greater", "0xffffffffU > -2", "1"),
        ("reverse_equal", "0xffffffffU == -1", "1"),
        ("uint_quotient", "-1 / 3U", "1431655765U"),
        ("uint_remainder", "-1 % 3U", "0U"),
        ("uint_even_quotient", "-2 / 2U", "2147483647U"),
        ("uint_even_remainder", "-2 % 3U", "2U"),
        ("uint_max_divisor", "-1 / 0xffffffffU", "1U"),
        ("uint_max_modulus", "-1 % 0xffffffffU", "0U"),
        ("negative_divisor", "0x80000000U / -1", "0U"),
        ("negative_modulus", "0x80000000U % -1", "0x80000000U"),
        ("long_represents_uint", "-1L < 0xffffffffU", "1"),
        ("long_and_uint_equal", "-1L == 0xffffffffU", "0"),
        ("long_uint_sum", "-1L + 0xffffffffU", "4294967294L"),
        ("long_uint_difference", "-1L - 0xffffffffU", "-4294967296L"),
        ("uint_long_division", "0xffffffffU / 2L", "2147483647L"),
        ("uint_long_remainder", "0xffffffffU % 2L", "1L"),
        ("ulong_converts_long", "-1L == 0xffffffffffffffffUL", "1"),
        ("ulong_quotient", "-1L / 3UL", "6148914691236517205UL"),
        ("ulong_remainder", "-1L % 3UL", "0UL"),
        ("ulong_wrap_add", "0xffffffffffffffffUL + 1UL", "0UL"),
        ("ulong_wrap_subtract", "0UL - 1UL", "0xffffffffffffffffUL"),
        ("uint_wrap_multiply", "0x80000000U * 2U", "0U"),
        ("uchar_promotion", "(unsigned char)255 + 1", "256"),
        ("ushort_promotion", "(unsigned short)65535 + 1", "65536"),
        ("char_signed_promotion", "-(signed char)-128", "128"),
        ("short_signed_promotion", "-(short)-32768", "32768"),
        (
            "narrow_mixed_equality",
            "(signed char)-1 == 0xffffffffU",
            "1",
        ),
        ("unsigned_negation", "-0x80000000U", "0x80000000U"),
        (
            "unsigned_long_negation",
            "-0x8000000000000000UL",
            "0x8000000000000000UL",
        ),
        ("complement_uint", "~0U / 3U", "1431655765U"),
        ("bitwise_mixed", "(-1 ^ 0xffffffffU)", "0U"),
        ("shift_after_unsigned_add", "(-1 + 0U) >> 1", "2147483647U"),
        ("shift_uint_last_bit", "1U << 31", "0x80000000U"),
        ("shift_ulong_last_bit", "1UL << 63", "0x8000000000000000UL"),
        ("shift_signed_long", "1L << 62", "4611686018427387904L"),
        ("shift_wide_count", "0x80000000U >> 31UL", "1U"),
        ("shift_promoted_uchar", "(unsigned char)128 << 8", "32768"),
        ("shift_signed_negative", "(-2147483647-1) >> 31", "-1"),
        (
            "conditional_conversion",
            "(1 ? -1 : 0U) == 0xffffffffU",
            "1",
        ),
        ("logical_short_circuit", "0 && (1 << 32)", "0"),
        ("conditional_short_circuit", "1 ? 7 : (1 / 0)", "7"),
    ] {
        cases.push(Case::new(
            format!("integer_constant_{name}"),
            format!("static and runtime target result: {expression}"),
            format!(
                "int folded=(({expression})==({expected}));\
                int main(void){{\
                    return folded&&(({expression})==({expected}));\
                }}"
            ),
            Expectation::Exit(1),
        ));
    }
    // Separate contexts use the ordinary evaluator instead of static leaf folding.
    for (name, source, expected) in [
        (
            "enum_comparison",
            "enum E{A=(-1==0xffffffffU)};int main(void){return A;}",
            1,
        ),
        (
            "enum_division",
            "enum E{A=(-1/0xffffffffU)};int main(void){return A;}",
            1,
        ),
        (
            "array_bound",
            "int a[(-1==0xffffffffU)+1];int main(void){return sizeof(a);}",
            8,
        ),
        (
            "local_array_bound",
            "int main(void){int a[(-1%3U)+2];return sizeof(a);}",
            8,
        ),
        (
            "switch_case",
            "int main(void){switch(1){case -1==0xffffffffU:return 7;default:return 3;}}",
            7,
        ),
        (
            "bitfield_width",
            "struct S{unsigned x:(-1==0xffffffffU)+1;};struct S s={3};int main(void){return s.x;}",
            3,
        ),
        (
            "null_integer_arm",
            "int x=7;int *p=1?&x:(-1!=0xffffffffU);int main(void){return *p;}",
            7,
        ),
        (
            "null_void_arm",
            "int x=9;int *p=0?(void*)(-1!=0xffffffffU):&x;int main(void){return *p;}",
            9,
        ),
        (
            "static_address_offset",
            "int a[]={3,7};int *p=a+(-1/0xffffffffU);int main(void){return *p;}",
            7,
        ),
        (
            "nested_numeric_pointer",
            "unsigned long n=(unsigned long)((int*)16+1)+(-1%3U);int main(void){return n;}",
            20,
        ),
    ] {
        cases.push(Case::new(
            format!("integer_constant_{name}"),
            name,
            source,
            Expectation::Exit(expected),
        ));
    }
    // Invalid evaluated integer constants must fail before hardware wrap or divide.
    for (name, expression) in [
        ("add_overflow", "2147483647+1"),
        ("subtract_overflow", "(-2147483647-1)-1"),
        ("multiply_overflow", "1073741824*2"),
        ("long_add_overflow", "9223372036854775807L+1L"),
        ("long_multiply_overflow", "4611686018427387904L*2L"),
        ("negate_int_min", "-(-2147483647-1)"),
        ("negate_long_min", "-(-9223372036854775807L-1)"),
        ("divide_int_min", "(-2147483647-1)/-1"),
        ("remainder_int_min", "(-2147483647-1)%-1"),
        ("divide_long_min", "(-9223372036854775807L-1L)/-1L"),
        ("remainder_long_min", "(-9223372036854775807L-1L)%-1L"),
        ("divide_zero", "1/0U"),
        ("remainder_zero", "1%0U"),
        ("shift_int_width", "1<<32"),
        ("shift_uint_width", "1U>>32"),
        ("shift_promoted_width", "(unsigned char)1<<32"),
        ("shift_long_width", "1L<<64"),
        ("shift_negative", "1>>-1"),
        ("shift_wide_count_not_narrowed", "1<<0x100000000L"),
        ("shift_unsigned_count_not_narrowed", "1U>>0x100000000UL"),
    ] {
        cases.push(Case::new(
            format!("integer_constant_reject_{name}"),
            format!("reject invalid evaluated constant: {expression}"),
            format!("long x={expression};int main(void){{return 0;}}"),
            Expectation::Reject("E011".into()),
        ));
    }
    cases
}
