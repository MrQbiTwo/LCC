// Exact transcripts for the unchanged primary application acceptance fixture.
use crate::{Case, Expectation};

const SOURCE: &str = include_str!("../../fixtures/calculator/original.c");
const HEADER: &str = "=== Calculator ===\n\n";
const MENU: &str = "Choose an operation:\n1 - Addition (+)\n2 - Subtraction (-)\n3 - Multiplication (*)\n4 - Division (/)\n5 - Modulo (%)\n6 - Power\nYour choice: ";
const NUMBERS: &str = "Enter first number: Enter second number: ";
const CONTINUE: &str = "\nContinue? (y/n): \n";

fn iteration(result: &str) -> String {
    format!("{MENU}{NUMBERS}{result}\n{CONTINUE}")
}

fn case(id: &str, purpose: &str, input: &str, output: String) -> Case {
    Case::new(
        format!("calculator_{id}"),
        purpose,
        SOURCE,
        Expectation::Io(input.into(), output, 0),
    )
}

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (id, purpose, input, result) in [
        (
            "addition",
            "decimal addition and two-digit result formatting",
            "1\n2.5\n3.25\nn\n",
            "2.50 + 3.25 = 5.75",
        ),
        (
            "subtraction",
            "negative subtraction result",
            "2\n2.25\n5.5\nn\n",
            "2.25 - 5.50 = -3.25",
        ),
        (
            "multiplication",
            "signed decimal multiplication",
            "3\n-1.5\n4\nn\n",
            "-1.50 * 4.00 = -6.00",
        ),
        (
            "division",
            "division with rounded fractional result",
            "4\n7\n3\nn\n",
            "7.00 / 3.00 = 2.33",
        ),
        (
            "modulo",
            "integer casts, remainder and literal percent",
            "5\n17\n5\nn\n",
            "17 % 5 = 2",
        ),
        (
            "negative_modulo",
            "signed remainder retains dividend sign",
            "5\n-17\n5\nn\n",
            "-17 % 5 = -2",
        ),
        (
            "power",
            "nonnegative integer exponent loop",
            "6\n1.5\n3\nn\n",
            "1.50 ^ 3 = 3.38",
        ),
        (
            "zero_power",
            "zero exponent leaves multiplicative identity",
            "6\n0\n0\nn\n",
            "0.00 ^ 0 = 1.00",
        ),
        (
            "division_zero",
            "division rejects a zero divisor",
            "4\n7\n0\nn\n",
            "Error: division by zero!",
        ),
        (
            "division_negative_zero",
            "division rejects signed zero",
            "4\n7\n-0\nn\n",
            "Error: division by zero!",
        ),
        (
            "modulo_zero",
            "integer modulo rejects a zero divisor",
            "5\n7\n0\nn\n",
            "Error: division by zero!",
        ),
        (
            "modulo_fractional_first",
            "modulo rejects a fractional dividend",
            "5\n7.5\n2\nn\n",
            "Modulo works only with integers!",
        ),
        (
            "modulo_fractional_second",
            "modulo rejects a fractional divisor",
            "5\n7\n2.5\nn\n",
            "Modulo works only with integers!",
        ),
        (
            "negative_exponent",
            "power rejects a negative exponent",
            "6\n2\n-1\nn\n",
            "Exponent must be a non-negative integer!",
        ),
        (
            "fractional_exponent",
            "power rejects a fractional exponent",
            "6\n2\n1.5\nn\n",
            "Exponent must be a non-negative integer!",
        ),
        (
            "invalid_operation",
            "switch default reports an invalid operation",
            "7\n2\n3\nn\n",
            "Invalid operation choice!",
        ),
        (
            "whitespace",
            "scanf accepts tabs and blank lines between values",
            " \t1\n\n 2.5\t\n3.25\nn\n",
            "2.50 + 3.25 = 5.75",
        ),
    ] {
        cases.push(case(
            id,
            purpose,
            input,
            format!("{HEADER}{}Goodbye!\n", iteration(result)),
        ));
    }
    cases.push(case(
        "continue_both_cases",
        "both y and Y repeat using shared scanf/getchar state",
        "1\n1\n2\ny\n2\n7\n3\nY\n3\n2\n4\nn\n",
        format!(
            "{HEADER}{}{}{}Goodbye!\n",
            iteration("1.00 + 2.00 = 3.00"),
            iteration("7.00 - 3.00 = 4.00"),
            iteration("2.00 * 4.00 = 8.00")
        ),
    ));
    // Retry is defined only after a successful iteration initialized continue_calc.
    let first = iteration("1.00 + 2.00 = 3.00");
    let last = iteration("3.00 + 4.00 = 7.00");
    for (id, purpose, middle_input, failure) in [
        (
            "retry_operation",
            "invalid operation input consumes the rest of its line",
            "bad input\n",
            "Input error! Enter a number.\n",
        ),
        (
            "retry_first_number",
            "invalid first number recovers after initialized continuation",
            "2\nbad input\n",
            "Enter first number: Input error!\n",
        ),
        (
            "retry_second_number",
            "invalid second number recovers after initialized continuation",
            "2\n5\nbad input\n",
            "Enter first number: Enter second number: Input error!\n",
        ),
    ] {
        cases.push(case(
            id,
            purpose,
            &format!("1\n1\n2\ny\n{middle_input}1\n3\n4\nn\n"),
            format!("{HEADER}{first}{MENU}{failure}{last}Goodbye!\n"),
        ));
    }
    // The supplied application spins at EOF while seeking a newline; retain it.
    for (id, input, prefix) in [
        (
            "eof_operation",
            "",
            format!("{HEADER}{MENU}Input error! Enter a number.\n"),
        ),
        (
            "eof_first_number",
            "1\n",
            format!("{HEADER}{MENU}Enter first number: Input error!\n"),
        ),
        (
            "eof_second_number",
            "1\n2\n",
            format!("{HEADER}{MENU}{NUMBERS}Input error!\n"),
        ),
        (
            "eof_before_continue",
            "1\n2\n3",
            format!("{HEADER}{MENU}{NUMBERS}2.00 + 3.00 = 5.00\n\nContinue? (y/n): "),
        ),
    ] {
        cases.push(Case::new(
            format!("calculator_{id}"),
            "unchanged application's newline-draining loop remains an expected EOF timeout",
            SOURCE,
            Expectation::IoTimeout(input.into(), prefix),
        ));
    }
    cases
}
