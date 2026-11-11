// Physical bitfield records workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "bitfield_records_01",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 1"
        ),
        fixture!(
            "bitfield_records_02",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 2"
        ),
        fixture!(
            "bitfield_records_03",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 3"
        ),
        fixture!(
            "bitfield_records_04",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 4"
        ),
        fixture!(
            "bitfield_records_05",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 5"
        ),
        fixture!(
            "bitfield_records_06",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 6"
        ),
        fixture!(
            "bitfield_records_07",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 7"
        ),
        fixture!(
            "bitfield_records_08",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 8"
        ),
        fixture!(
            "bitfield_records_09",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 9"
        ),
        fixture!(
            "bitfield_records_10",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 10"
        ),
        fixture!(
            "bitfield_records_11",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 11"
        ),
        fixture!(
            "bitfield_records_12",
            "bitfield_records",
            "Packed bit-field assignments, extraction and truncation; scale point 12"
        ),
    ]
}
