// Physical binary80 series workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "binary80_series_01",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 1"
        ),
        fixture!(
            "binary80_series_02",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 2"
        ),
        fixture!(
            "binary80_series_03",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 3"
        ),
        fixture!(
            "binary80_series_04",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 4"
        ),
        fixture!(
            "binary80_series_05",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 5"
        ),
        fixture!(
            "binary80_series_06",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 6"
        ),
        fixture!(
            "binary80_series_07",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 7"
        ),
        fixture!(
            "binary80_series_08",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 8"
        ),
        fixture!(
            "binary80_series_09",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 9"
        ),
        fixture!(
            "binary80_series_10",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 10"
        ),
        fixture!(
            "binary80_series_11",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 11"
        ),
        fixture!(
            "binary80_series_12",
            "binary80_series",
            "Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 12"
        ),
    ]
}
