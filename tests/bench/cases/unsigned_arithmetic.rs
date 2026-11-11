// Physical unsigned arithmetic workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "unsigned_arithmetic_01",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 1"
        ),
        fixture!(
            "unsigned_arithmetic_02",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 2"
        ),
        fixture!(
            "unsigned_arithmetic_03",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 3"
        ),
        fixture!(
            "unsigned_arithmetic_04",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 4"
        ),
        fixture!(
            "unsigned_arithmetic_05",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 5"
        ),
        fixture!(
            "unsigned_arithmetic_06",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 6"
        ),
        fixture!(
            "unsigned_arithmetic_07",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 7"
        ),
        fixture!(
            "unsigned_arithmetic_08",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 8"
        ),
        fixture!(
            "unsigned_arithmetic_09",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 9"
        ),
        fixture!(
            "unsigned_arithmetic_10",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 10"
        ),
        fixture!(
            "unsigned_arithmetic_11",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 11"
        ),
        fixture!(
            "unsigned_arithmetic_12",
            "unsigned_arithmetic",
            "Wrapping unsigned multiply/add recurrences; scale point 12"
        ),
    ]
}
