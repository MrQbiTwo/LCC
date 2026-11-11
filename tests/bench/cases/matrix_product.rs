// Physical matrix product workloads and independent expectations.
use super::Case;

pub(super) fn cases() -> Vec<Case> {
    vec![
        fixture!(
            "matrix_product_01",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 1"
        ),
        fixture!(
            "matrix_product_02",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 2"
        ),
        fixture!(
            "matrix_product_03",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 3"
        ),
        fixture!(
            "matrix_product_04",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 4"
        ),
        fixture!(
            "matrix_product_05",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 5"
        ),
        fixture!(
            "matrix_product_06",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 6"
        ),
        fixture!(
            "matrix_product_07",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 7"
        ),
        fixture!(
            "matrix_product_08",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 8"
        ),
        fixture!(
            "matrix_product_09",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 9"
        ),
        fixture!(
            "matrix_product_10",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 10"
        ),
        fixture!(
            "matrix_product_11",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 11"
        ),
        fixture!(
            "matrix_product_12",
            "matrix_product",
            "Two-dimensional matrix multiply and reduction; scale point 12"
        ),
    ]
}
