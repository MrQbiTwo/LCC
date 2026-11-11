// Decimal conversion vectors use Rust's independent parser and exact binary80 cases.
use crate::{Case, Expectation};

// Produce format boundaries, ties, exponent ranges, and malformed spellings.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let mut values: Vec<String> = [
        "0",
        "-0",
        "1",
        "-1",
        "0.1",
        "0.5",
        "1.5",
        "123456789.125",
        "1.00000000000000011102230246251565404236316680908203125",
        "1.00000000000000033306690738754696212708950042724609375",
        "1.000000059604644775390625",
        "1.000000178813934326171875",
        "4.9406564584124654e-324",
        "2.2250738585072014e-308",
        "1.7976931348623157e308",
        "1.7976931348623159e308",
        "1.401298464324817e-45",
        "1.1754943508222875e-38",
        "3.4028234663852886e38",
        "1e-10000",
        "1e10000",
        "-1e-10000",
        "-1e10000",
        "000000.00000000000125",
        ".25",
        "12.",
        "+3.75",
        "999999999999999999999999999.5",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    for exponent in (-360..=360).step_by(30) {
        values.push(format!("1234567.8901234567890123456789e{exponent}"));
    }
    for (format, precision) in [(0, "binary32"), (1, "binary64")] {
        for value in &values {
            let (bits, infinite, zero) = if format == 0 {
                let number = value.parse::<f32>().unwrap();
                (number.to_bits() as u64, number.is_infinite(), number == 0.0)
            } else {
                let number = value.parse::<f64>().unwrap();
                (number.to_bits(), number.is_infinite(), number == 0.0)
            };
            let exact_zero = matches!(value.as_str(), "0" | "-0");
            let status = if infinite {
                2
            } else if zero && !exact_zero {
                3
            } else {
                0
            };
            let number = cases.len() + 1;
            cases.push(Case::new(
                format!("decimal_{number:03}"),
                format!("{precision} rounding for {value}"),
                String::new(),
                Expectation::Decimal(value.clone(), format, bits, 0, status),
            ));
        }
    }
    for (value, low, high, status) in [
        ("0", 0, 0, 0),
        ("-0", 0, 0x8000, 0),
        ("1", 0x8000000000000000, 0x3fff, 0),
        ("-1.5", 0xc000000000000000, 0xbfff, 0),
        ("2", 0x8000000000000000, 0x4000, 0),
        ("0.5", 0x8000000000000000, 0x3ffe, 0),
        (
            "1.000000000000000000108420217248550443400745280086994171142578125",
            0x8000000000000001,
            0x3fff,
            0,
        ),
        (
            "1.0000000000000000000542101086242752217003726400434970855712890625",
            0x8000000000000000,
            0x3fff,
            0,
        ),
        (
            "1.0000000000000000001626303258728256651011179201304912567138671875",
            0x8000000000000002,
            0x3fff,
            0,
        ),
        ("1e10000", 0x8000000000000000, 0x7fff, 2),
        ("-1e10000", 0x8000000000000000, 0xffff, 2),
        ("1e-10000", 0, 0, 3),
        ("-1e-10000", 0, 0x8000, 3),
    ] {
        let number = cases.len() + 1;
        cases.push(Case::new(
            format!("decimal_{number:03}"),
            format!("binary80 exact vector {value}"),
            String::new(),
            Expectation::Decimal(value.into(), 2, low, high, status),
        ));
    }
    for value in [
        "", ".", "1e", "1e+", "1e-", "1.2.3", "--1", "1x", "0x1p0", "1 2",
    ] {
        let number = cases.len() + 1;
        cases.push(Case::new(
            format!("decimal_{number:03}"),
            format!("reject decimal spelling {value:?}"),
            String::new(),
            Expectation::Decimal(value.into(), 1, 0, 0, 1),
        ));
    }
    cases
}
