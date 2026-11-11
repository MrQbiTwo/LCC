// Physical recursive calls workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "recursive_calls_01",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 1"
        ),
        fixture!(
            "recursive_calls_02",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 2"
        ),
        fixture!(
            "recursive_calls_03",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 3"
        ),
        fixture!(
            "recursive_calls_04",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 4"
        ),
        fixture!(
            "recursive_calls_05",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 5"
        ),
        fixture!(
            "recursive_calls_06",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 6"
        ),
        fixture!(
            "recursive_calls_07",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 7"
        ),
        fixture!(
            "recursive_calls_08",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 8"
        ),
        fixture!(
            "recursive_calls_09",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 9"
        ),
        fixture!(
            "recursive_calls_10",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 10"
        ),
        fixture!(
            "recursive_calls_11",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 11"
        ),
        fixture!(
            "recursive_calls_12",
            "recursive_calls",
            "Recursive Fibonacci call trees; scale point 12"
        ),
    ]
}
