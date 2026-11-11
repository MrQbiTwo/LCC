// Physical binary64 series workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "binary64_series_01",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 1"
        ),
        fixture!(
            "binary64_series_02",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 2"
        ),
        fixture!(
            "binary64_series_03",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 3"
        ),
        fixture!(
            "binary64_series_04",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 4"
        ),
        fixture!(
            "binary64_series_05",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 5"
        ),
        fixture!(
            "binary64_series_06",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 6"
        ),
        fixture!(
            "binary64_series_07",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 7"
        ),
        fixture!(
            "binary64_series_08",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 8"
        ),
        fixture!(
            "binary64_series_09",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 9"
        ),
        fixture!(
            "binary64_series_10",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 10"
        ),
        fixture!(
            "binary64_series_11",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 11"
        ),
        fixture!(
            "binary64_series_12",
            "binary64_series",
            "Exact binary64 arithmetic across function boundaries; scale point 12"
        ),
    ]
}
