// Physical bit mixing workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "bit_mixing_01",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 1"
        ),
        fixture!(
            "bit_mixing_02",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 2"
        ),
        fixture!(
            "bit_mixing_03",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 3"
        ),
        fixture!(
            "bit_mixing_04",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 4"
        ),
        fixture!(
            "bit_mixing_05",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 5"
        ),
        fixture!(
            "bit_mixing_06",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 6"
        ),
        fixture!(
            "bit_mixing_07",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 7"
        ),
        fixture!(
            "bit_mixing_08",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 8"
        ),
        fixture!(
            "bit_mixing_09",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 9"
        ),
        fixture!(
            "bit_mixing_10",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 10"
        ),
        fixture!(
            "bit_mixing_11",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 11"
        ),
        fixture!(
            "bit_mixing_12",
            "bit_mixing",
            "Shift and XOR mixing with exact unsigned semantics; scale point 12"
        ),
    ]
}
