// Central benchmark registry; each entry names one independently verified C file.
pub struct Case {
    pub id: &'static str,
    pub family: &'static str,
    pub purpose: &'static str,
    pub source: &'static str,
    pub expected: i32,
}

macro_rules! fixture {
    ($id:literal, $family:literal, $purpose:literal) => {
        Case {
            id: $id,
            family: $family,
            purpose: $purpose,
            expected: 0,
            source: include_str!(concat!("../../fixtures/perf/", $family, "/", $id, ".c")),
        }
    };
}

#[path = "cases/array_search.rs"]
mod array_search;
#[path = "cases/binary64_series.rs"]
mod binary64_series;
#[path = "cases/binary80_series.rs"]
mod binary80_series;
#[path = "cases/bit_mixing.rs"]
mod bit_mixing;
#[path = "cases/bitfield_records.rs"]
mod bitfield_records;
#[path = "cases/byte_pointers.rs"]
mod byte_pointers;
#[path = "cases/function_chain.rs"]
mod function_chain;
#[path = "cases/function_pointers.rs"]
mod function_pointers;
#[path = "cases/insertion_sort.rs"]
mod insertion_sort;
#[path = "cases/macro_expansion.rs"]
mod macro_expansion;
#[path = "cases/matrix_product.rs"]
mod matrix_product;
#[path = "cases/nested_loops.rs"]
mod nested_loops;
#[path = "cases/record_declarations.rs"]
mod record_declarations;
#[path = "cases/record_values.rs"]
mod record_values;
#[path = "cases/recursive_calls.rs"]
mod recursive_calls;
#[path = "cases/static_initializers.rs"]
mod static_initializers;
#[path = "cases/switch_dispatch.rs"]
mod switch_dispatch;
#[path = "cases/union_layout.rs"]
mod union_layout;
#[path = "cases/unsigned_arithmetic.rs"]
mod unsigned_arithmetic;
#[path = "cases/variadic_integers.rs"]
mod variadic_integers;

// One entry per family; all consumers share this registry.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    cases.extend(unsigned_arithmetic::cases());
    cases.extend(bit_mixing::cases());
    cases.extend(switch_dispatch::cases());
    cases.extend(nested_loops::cases());
    cases.extend(function_chain::cases());
    cases.extend(recursive_calls::cases());
    cases.extend(array_search::cases());
    cases.extend(insertion_sort::cases());
    cases.extend(matrix_product::cases());
    cases.extend(record_values::cases());
    cases.extend(union_layout::cases());
    cases.extend(bitfield_records::cases());
    cases.extend(function_pointers::cases());
    cases.extend(byte_pointers::cases());
    cases.extend(binary64_series::cases());
    cases.extend(binary80_series::cases());
    cases.extend(macro_expansion::cases());
    cases.extend(static_initializers::cases());
    cases.extend(variadic_integers::cases());
    cases.extend(record_declarations::cases());
    cases
}
