// Check the benchmark corpus independently of performance measurements.
#[path = "cases.rs"]
mod cases;
#[path = "../support/process.rs"]
mod process;
use std::{env, fs, path::PathBuf, time::Duration};
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "validate_cases <compiler> <new_work_directory>"
    );
    let compiler = fs::canonicalize(&args[1]).unwrap();
    let directory = PathBuf::from(&args[2]);
    fs::create_dir(&directory).expect("new validation directory");
    let mut report = String::from("case,family,bytes,result\n");
    let mut failures = 0;
    let cases = cases::cases();
    for case in &cases {
        let source = directory.join(format!("{}.c", case.id));
        let binary = directory.join(format!("{}.elf", case.id));
        fs::write(&source, case.source).unwrap();
        let output = process::invoke(
            &compiler,
            &[
                source.display().to_string(),
                "-o".into(),
                binary.display().to_string(),
            ],
            vec![],
            Duration::from_secs(30),
        )
        .unwrap();
        let mut okay = output.status == Some(0) && !output.timed_out && !output.truncated;
        if okay {
            let run = process::invoke(&binary, &[], vec![], Duration::from_secs(5)).unwrap();
            okay = run.status == Some(case.expected)
                && !run.timed_out
                && !run.truncated
                && run.stdout.is_empty()
                && run.stderr.is_empty();
            if !okay {
                eprintln!("{} execution: {run:?}", case.id);
            }
        } else {
            eprintln!("{} compilation: {output:?}", case.id);
        }
        if !okay {
            failures += 1;
            eprintln!("Purpose: {}", case.purpose);
        }
        report.push_str(&format!(
            "{},{},{},{}\n",
            case.id,
            case.family,
            case.source.len(),
            if okay { "pass" } else { "fail" }
        ));
        fs::write(directory.join("results.csv"), &report).unwrap();
    }
    println!(
        "{}/{} benchmark fixtures pass",
        cases.len() - failures,
        cases.len()
    );
    if failures != 0 {
        std::process::exit(1);
    }
}
