use crate::{Case, Expectation};

// Register CLI cases independently of source compilation cases.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (index, (args, code, stdout, purpose)) in [
        (vec!["-v"], 0, "", "short version"),
        (vec!["-version"], 0, "", "long version"),
        (vec!["-v", "-v"], 0, "", "idempotent version"),
        (
            vec!["nonexistent.c", "-v"],
            0,
            "",
            "information suppresses file access",
        ),
        (vec!["-S", "-v"], 0, "", "information with output mode"),
        (
            vec!["-o", "unused", "-v"],
            0,
            "",
            "information with output path",
        ),
        (vec![], 2, "", "missing arguments"),
        (vec!["-S"], 2, "", "missing input"),
        (vec!["-o"], 2, "", "missing output operand"),
        (vec!["-v", "--bad"], 2, "", "invalid option with version"),
        (
            vec!["-o", "a", "-o", "b", "-v"],
            2,
            "",
            "duplicate output option",
        ),
        (
            vec!["a.c", "b.c", "-v"],
            2,
            "",
            "multiple inputs in information mode",
        ),
        (vec!["file.txt"], 2, "", "unsupported input suffix"),
    ]
    .into_iter()
    .enumerate()
    {
        let number = index + 1;
        cases.push(Case::new(
            format!("cli_{number:03}"),
            purpose,
            "",
            if code == 0 {
                Expectation::CliVersion(args.iter().map(|s| s.to_string()).collect())
            } else {
                Expectation::Cli(
                    args.iter().map(|s| s.to_string()).collect(),
                    code,
                    stdout.into(),
                )
            },
        ));
    }
    cases
}
