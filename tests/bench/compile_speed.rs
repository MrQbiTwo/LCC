// Compare compilation wall time on the separately registered supported C inputs.
#[path = "speed/archive.rs"]
mod archive;
mod cases;
#[path = "speed/methods.rs"]
mod methods;
#[path = "../support/process.rs"]
mod process;

use archive::{copy_contents, csv, hash, Archive};
use methods::{compile, verify, Method};
use std::{env, fs, io::Write, path::PathBuf};

// IDs become file names; all expected results use the portable process exit range.
fn validate(cases: &[cases::Case]) -> Result<(), String> {
    let mut seen = std::collections::HashSet::new();
    if cases.len() <= 200 {
        return Err("registry must contain more than 200 cases".into());
    }
    for case in cases {
        if case.id.is_empty()
            || case.id.len() > 128
            || !case
                .id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
        {
            return Err(format!("unsafe case ID: {:?}", case.id));
        }
        if !seen.insert(case.id) {
            return Err(format!("duplicate case ID: {}", case.id));
        }
        if !(0..=255).contains(&case.expected)
            || case.source.is_empty()
            || case.family.is_empty()
            || case.purpose.is_empty()
        {
            return Err(format!("incomplete case: {}", case.id));
        }
    }
    Ok(())
}

// Rotate and reverse order across cases/trials without concurrent compiler work.
fn order(case: usize, round: usize, count: usize) -> Vec<usize> {
    let start = (case + round) % count;
    (0..count)
        .map(|offset| {
            if (case + round) % 2 == 0 {
                (start + offset) % count
            } else {
                (start + count - offset) % count
            }
        })
        .collect()
}

// Median handles even trial counts; nearest-rank p95 is labeled in metadata.
fn summarize(values: &mut [f64]) -> (f64, f64) {
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    let median = if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    };
    (median, values[(values.len() * 95).div_ceil(100) - 1])
}

// Exact bytes may reuse prior verification; changed outputs are always executed.
fn run_case(
    case: &cases::Case,
    case_index: usize,
    methods: &[Method],
    trials: usize,
    archive: &mut Archive,
) -> Result<(), String> {
    let directory = archive.work.join(case.id);
    fs::create_dir(&directory).map_err(|e| format!("create case directory: {e}"))?;
    let input = directory.join(format!("{}.c", case.id));
    fs::write(&input, case.source).map_err(|e| format!("write input: {e}"))?;
    let outputs: Vec<PathBuf> = methods
        .iter()
        .map(|method| directory.join(format!("{}.{}", method.name, method.extension())))
        .collect();
    let mut samples = vec![Vec::<f64>::new(); methods.len()];
    let mut verified = vec![None::<Vec<u8>>; methods.len()];
    for round in 0..=trials {
        for index in order(case_index, round, methods.len()) {
            let method = &methods[index];
            let elapsed = compile(method, &input, &outputs[index])?;
            let bytes = fs::read(&outputs[index]).map_err(|e| format!("read output: {e}"))?;
            if verified[index].as_ref() != Some(&bytes) {
                verify(method, &outputs[index], &directory, case.expected)?;
                verified[index] = Some(bytes.clone());
            }
            if round != 0 {
                samples[index].push(elapsed);
                writeln!(
                    archive.samples,
                    "{},{},{},{},{},{elapsed:.6},{},{:016x}",
                    csv(case.id),
                    csv(case.family),
                    case.source.len(),
                    method.name,
                    round - 1,
                    bytes.len(),
                    hash(&bytes)
                )
                .map_err(|e| e.to_string())?;
                archive.samples.flush().map_err(|e| e.to_string())?;
            }
        }
    }
    for (index, values) in samples.iter_mut().enumerate() {
        let (median, p95) = summarize(values);
        let output_size = fs::metadata(&outputs[index])
            .map_err(|e| e.to_string())?
            .len();
        let row = format!(
            "{},{},{},{},{},{},{median:.6},{p95:.6},{:.6},{:.6},{output_size},{:.6}",
            csv(case.id),
            csv(case.family),
            case.source.len(),
            case.source.lines().count(),
            methods[index].name,
            values.len(),
            values[0],
            values[values.len() - 1],
            case.source.len() as f64 / 1048576.0 / (median / 1000.0)
        );
        writeln!(archive.summary, "{row}").map_err(|e| e.to_string())?;
    }
    archive.summary.flush().map_err(|e| e.to_string())?;
    println!(
        "verified {}: six methods, one warmup and {trials} trials",
        case.id
    );
    std::io::stdout().flush().map_err(|e| e.to_string())
}

// Sources and measurements reach persistent storage before the Linux process exits.
fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if !(4..=6).contains(&args.len()) {
        return Err(
            "usage: compile_speed <lcc> <new_workdir> <new_archive> [trials] [filter]".into(),
        );
    }
    let trials = args
        .get(4)
        .map(|s| s.parse::<usize>())
        .transpose()
        .map_err(|_| "trials must be an integer")?
        .unwrap_or(3);
    if !(1..=100).contains(&trials) {
        return Err("trials must be in 1..=100".into());
    }
    let filter = args.get(5).map(String::as_str).unwrap_or("");
    let registry = cases::cases();
    validate(&registry)?;
    let selected: Vec<_> = registry
        .into_iter()
        .filter(|case| case.id.contains(filter) || case.family.contains(filter))
        .collect();
    if selected.is_empty() {
        return Err(format!("no cases match {filter:?}"));
    }
    let mut archive = Archive::create(&args[2], &args[3])?;
    let result = (|| {
        let lcc = archive.work.join("lcc");
        copy_contents(std::path::Path::new(&args[1]), &lcc, true)?;
        let methods = methods::all(lcc);
        archive.inputs(&selected)?;
        archive.metadata(&methods, trials, filter, &args)?;
        for (index, case) in selected.iter().enumerate() {
            if let Err(error) = run_case(case, index, &methods, trials, &mut archive) {
                archive.failure(case.id, &error)?;
                return Err(format!("{}: {error}", case.id));
            }
        }
        fs::write(
            archive.path.join("complete.txt"),
            format!(
            "Verified {} cases, six methods, one warmup and {trials} measured trials per method.\n",
            selected.len()),
        )
        .map_err(|e| e.to_string())?;
        println!(
            "complete: {} cases; archive {}",
            selected.len(),
            archive.path.display()
        );
        Ok(())
    })();
    if let Err(error) = &result {
        let _ = fs::write(archive.path.join("failed.txt"), format!("{error}\n"));
    }
    result
}

fn main() {
    if let Err(error) = run() {
        eprintln!("compile_speed: {error}");
        std::process::exit(1);
    }
}
