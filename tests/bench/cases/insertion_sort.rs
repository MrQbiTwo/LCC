// Physical insertion sort workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "insertion_sort_01",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 1"
        ),
        fixture!(
            "insertion_sort_02",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 2"
        ),
        fixture!(
            "insertion_sort_03",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 3"
        ),
        fixture!(
            "insertion_sort_04",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 4"
        ),
        fixture!(
            "insertion_sort_05",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 5"
        ),
        fixture!(
            "insertion_sort_06",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 6"
        ),
        fixture!(
            "insertion_sort_07",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 7"
        ),
        fixture!(
            "insertion_sort_08",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 8"
        ),
        fixture!(
            "insertion_sort_09",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 9"
        ),
        fixture!(
            "insertion_sort_10",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 10"
        ),
        fixture!(
            "insertion_sort_11",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 11"
        ),
        fixture!(
            "insertion_sort_12",
            "insertion_sort",
            "In-place insertion sorting and weighted checksums; scale point 12"
        ),
    ]
}
