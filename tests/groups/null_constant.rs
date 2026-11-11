// Optional null recognition must not turn unused arithmetic into a fatal error.
use crate::{Case, Expectation};

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    // Every bad operation is inside a pointer arm which the program never selects.
    // It is not a required integer constant expression: ordinary void/object
    // pointer composition remains legal when optional null recognition fails.
    for (index, (id, expression)) in [
        ("divide_zero", "1L/0L"),
        ("remainder_zero", "1L%0L"),
        ("int_add_overflow", "2147483647+1"),
        ("int_subtract_overflow", "(-2147483647-1)-1"),
        ("int_multiply_overflow", "1073741824*2"),
        ("int_negate_minimum", "-(-2147483647-1)"),
        ("int_divide_overflow", "(-2147483647-1)/-1"),
        ("int_remainder_overflow", "(-2147483647-1)%-1"),
        ("long_add_overflow", "9223372036854775807L+1L"),
        ("long_subtract_overflow", "(-9223372036854775807L-1L)-1L"),
        ("long_multiply_overflow", "4611686018427387904L*2L"),
        ("long_negate_minimum", "-(-9223372036854775807L-1L)"),
        ("long_divide_overflow", "(-9223372036854775807L-1L)/-1L"),
        ("long_remainder_overflow", "(-9223372036854775807L-1L)%-1L"),
        ("left_shift_negative", "1L<<-1"),
        ("right_shift_negative", "1L>>-1"),
        ("left_shift_width", "1UL<<64"),
        ("right_shift_width", "1UL>>64"),
        ("int_shift_width", "1U<<32"),
        ("double_to_int_range", "(int)1.0e100"),
        ("float_to_long_range", "(long)1.0e30f"),
        ("extended_to_unsigned_range", "(unsigned long)1.0e100L"),
    ]
    .into_iter()
    .enumerate()
    {
        let selection = if index % 2 == 0 {
            format!("0?(void*)({expression}):p")
        } else {
            format!("1?p:(void*)({expression})")
        };
        cases.push(Case::new(
            format!("null_constant_{}", id),
            "unused arithmetic failure falls back to ordinary object/void pointer composition",
            format!("int main(void){{int x=4;int *p=&x;int *q={selection};return *q!=4;}}"),
            Expectation::Exit(0),
        ));
    }
    for (id, purpose, source) in [
        (
            "unused_nested_conditional",
            "a failed nested arithmetic probe leaves the selected object intact",
            "int main(void){int x=6;int *p=&x;int *q=0?(void*)(1?1L/0L:0L):p;return *q!=6;}",
        ),
        (
            "unused_multiple_probes",
            "failed probes do not poison later successful null recognition",
            "int main(void){\
                int a[2]={\
                    3,7};\
                int *p=0?(void*)(1L/0L):a;\
                int *q=1?p:(void*)(1L%0L);\
                return *((1?q:(void*)(3-3))+1)!=7;\
            }",
        ),
        (
            "unused_qualified_object",
            "normal void composition retains const and volatile object qualifiers",
            "int main(void){\
                const volatile int x=9;\
                const volatile int *p=&x;\
                const volatile void *q=0?(void*)(1L/0L):p;\
                return *(const volatile int*)q!=9;\
            }",
        ),
        (
            "unused_record_pointer",
            "normal void composition accepts complete record object pointers",
            "struct S{\
                int x;\
            };\
            int main(void){\
                struct S s={11};\
                struct S *p=&s;\
                struct S *q=0?(void*)(1L/0L):p;\
                return q->x!=11;\
            }",
        ),
        (
            "unused_pointer_to_array",
            "normal void composition accepts pointers to complete arrays",
            "int main(void){\
                int a[2]={\
                    3,7};\
                int (*p)[2]=&a;\
                int (*q)[2]=1?p:(void*)(1L/0L);\
                return (*q)[1]!=7;\
            }",
        ),
    ] {
        cases.push(Case::new(
            format!("null_constant_{}", id),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }

    // Calling the result distinguishes a true null constant from generic void*.
    for (id, expression) in [
        ("zero_subtract", "7L-7L"),
        ("zero_unsigned_wrap", "0xffffffffU+1U"),
        ("zero_remainder", "27UL%9UL"),
        ("zero_bitwise", "(15U^15U)&31U"),
        ("zero_float_cast", "(int)0.75"),
        ("zero_extended_cast", "(long)0.875L"),
        ("zero_sizeof", "sizeof(long)-sizeof(long)"),
        ("zero_short_circuit", "0&&(1L/0L)"),
        ("zero_skipped_overflow", "0?(9223372036854775807L+1L):0"),
        ("zero_logical_or", "!(1||(1L/0L))"),
        ("zero_comparison", "3U>7U"),
        (
            "zero_nested_integer_cast",
            "(long)(unsigned short)(int)0.75",
        ),
    ] {
        cases.push(Case::new(
            format!("null_constant_{}", id),
            "computed zero retains function-pointer conditional type",
            format!("int f(int x){{return x+3;}}int main(void){{return (1?f:(void*)({expression}))(4)!=7;}}"),
            Expectation::Exit(0),
        ));
    }
    for (id, purpose, source) in [
        (
            "zero_left_function",
            "null recognition is symmetric for a left void-pointer arm",
            "int f(void){return 9;}int main(void){return (0?(void*)(2-2):f)()!=9;}",
        ),
        (
            "zero_selected_function_null",
            "a selected computed zero produces an actual null function pointer",
            "int f(void){return 9;}int main(void){int (*p)(void)=1?(void*)(2-2):f;return p!=0;}",
        ),
        (
            "zero_array_scaling",
            "computed null preserves pointed-to element size in pointer arithmetic",
            "int main(void){int a[3]={3,7,11};return *((0?(void*)(5-5):a)+2)!=11;}",
        ),
        (
            "zero_sizeof_side_effect",
            "sizeof admits side effects without evaluating them during null recognition",
            "int n;\
            int f(void){\
                return 9;\
            }\
            int main(void){\
                int (*p)(void)=1?f:(void*)(sizeof(n++)-sizeof(int));\
                return !(p()==9&&n==0);\
            }",
        ),
    ] {
        cases.push(Case::new(
            format!("null_constant_{}", id),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }

    for (id, expression) in [
        ("reject_nonzero", "1L"),
        ("reject_runtime_zero", "n"),
        ("reject_comma_zero", "(1,0)"),
        ("reject_float_arithmetic_zero", "(int)(0.5-0.5)"),
        ("reject_pointer_cast_zero", "(long)(void*)0"),
        ("reject_skipped_variable", "0&&n"),
        ("reject_skipped_assignment", "1?0:(n=0)"),
        ("reject_failed_probe", "1L/0L"),
        ("reject_failed_conversion", "(long)1.0e100"),
    ] {
        cases.push(Case::new(
            format!("null_constant_{}", id),
            "a non-null void pointer cannot compose with a function pointer",
            format!(
                "int n;\
                int f(void){{\
                    return 7;\
                }}\
                int main(void){{\
                    int (*p)(void)=1?f:(void*)({expression});\
                    return p();\
                }}"
            ),
            Expectation::Reject("E008".into()),
        ));
    }
    for (id, purpose, source) in [
        (
            "reject_const_void",
            "const void zero is a generic pointer rather than the special null form",
            "int f(void){return 7;}int main(void){int (*p)(void)=1?f:(const void*)(3-3);return p();}",
        ),
        (
            "reject_volatile_void",
            "volatile void zero cannot compose with a function pointer",
            "int f(void){return 7;}int main(void){int (*p)(void)=0?(volatile void*)(3-3):f;return p();}",
        ),
        (
            "reject_qualified_void_dereference",
            "qualified void zero keeps the conditional result void",
            "int main(void){int x=4;*(1?&x:(const void*)(3-3))=3;return 0;}",
        ),
        (
            "reject_non_ice_void_dereference",
            "a numerically zero non-ICE void pointer keeps the conditional result void",
            "int main(void){int x=4;*(1?&x:(void*)(int)(0.5-0.5))=3;return 0;}",
        ),
        (
            "reject_failed_probe_void_dereference",
            "arithmetic failure does not falsely recover an object result type",
            "int main(void){int x=4;*(1?&x:(void*)(1L/0L))=3;return 0;}",
        ),
    ] { cases.push(Case::new(
        format!("null_constant_{}", id),
        purpose,
        source,
        Expectation::Reject("E008".into()),
    )); }

    for (id, purpose, source) in [
        (
            "required_enum_divide",
            "required enum evaluation retains division-by-zero diagnostics",
            "enum E{N=1L/0L};int main(void){return 0;}",
        ),
        (
            "required_array_remainder",
            "required array evaluation retains remainder-by-zero diagnostics",
            "int a[1L%0L];int main(void){return 0;}",
        ),
        (
            "required_bitfield_overflow",
            "required bit-field evaluation retains overflow diagnostics",
            "struct S{unsigned n:2147483647+1;};int main(void){return 0;}",
        ),
        (
            "required_case_negate",
            "required case evaluation retains minimum-negation diagnostics",
            "int main(void){switch(0){case -(-2147483647-1):return 1;default:return 0;}}",
        ),
        (
            "required_enum_shift",
            "required enum evaluation retains invalid-shift diagnostics",
            "enum E{N=1U<<32};int main(void){return 0;}",
        ),
        (
            "required_enum_float_range",
            "required immediate floating conversion retains range diagnostics",
            "enum E{N=(int)1.0e100};int main(void){return 0;}",
        ),
        (
            "required_pointer_integer",
            "an integer arm must supply a valid null constant rather than arbitrary arithmetic",
            "int main(void){int x=4;int *p=1?&x:1L/0L;return 0;}",
        ),
        (
            "required_offset_index",
            "required offsetof index evaluation retains arithmetic diagnostics",
            "#include <stddef.h>\n\
            struct S{\
                int a[3];\
            };\
            int main(void){\
                return offsetof(struct S,a[1L/0L]);\
            }",
        ),
    ] {
        cases.push(Case::new(
            format!("null_constant_{}", id),
            purpose,
            source,
            Expectation::Reject("E011".into()),
        ));
    }
    cases
}
