// Compare two compiler snapshots on identical validated inputs and retain samples.
mod corpus;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

// Record only compiler wall time; input generation and execution checks are separate.
fn compile(compiler: &Path, source: &Path, output: &Path, assembly: bool) -> f64 {
    let mut command = Command::new(compiler);
    command.arg(source).arg("-o").arg(output);
    if assembly {
        command.arg("-S");
    }
    let start = Instant::now();
    let result = command.output().unwrap();
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    elapsed
}

// The archive is written before this process exits, including on temporary storage.
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        5,
        "compiler_compare <before> <after> <new_workdir> <new_archive>"
    );
    let work = PathBuf::from(&args[3]);
    let archive = PathBuf::from(&args[4]);
    fs::create_dir(&work).unwrap();
    fs::create_dir(&archive).unwrap();
    let compilers = [work.join("before"), work.join("after")];
    for (index, compiler) in compilers.iter().enumerate() {
        fs::copy(&args[index + 1], compiler).unwrap();
    }
    let mut raw = fs::File::create(archive.join("samples.csv")).unwrap();
    let mut summary = fs::File::create(archive.join("summary.csv")).unwrap();
    writeln!(raw, "functions,mode,compiler,trial,ms").unwrap();
    writeln!(
        summary,
        "functions,bytes,mode,before_ms,after_ms,speedup,before_output,after_output"
    )
    .unwrap();
    for count in [500, 5000] {
        let (text, expected) = corpus::corpus(count);
        let source = work.join(format!("functions_{count}.c"));
        fs::write(&source, &text).unwrap();
        // Archive contents without copying filesystem-specific permission metadata.
        fs::write(archive.join(source.file_name().unwrap()), &text).unwrap();
        for assembly in [false, true] {
            let mode = if assembly { "asm" } else { "elf" };
            let outputs = [
                work.join(format!("before_{count}.{mode}")),
                work.join(format!("after_{count}.{mode}")),
            ];
            for index in 0..2 {
                for _ in 0..2 {
                    compile(&compilers[index], &source, &outputs[index], assembly);
                }
                if !assembly {
                    assert_eq!(
                        Command::new(&outputs[index]).status().unwrap().code(),
                        Some(expected)
                    );
                }
            }
            let mut samples = [Vec::new(), Vec::new()];
            for trial in 0..15 {
                for offset in 0..2 {
                    let index = (trial + offset) % 2;
                    let ms = compile(&compilers[index], &source, &outputs[index], assembly);
                    samples[index].push(ms);
                    writeln!(raw, "{count},{mode},{index},{trial},{ms:.6}").unwrap();
                }
            }
            for sample in &mut samples {
                sample.sort_by(f64::total_cmp);
            }
            let row = format!(
                "{count},{},{mode},{:.6},{:.6},{:.3},{},{}",
                text.len(),
                samples[0][7],
                samples[1][7],
                samples[0][7] / samples[1][7],
                fs::metadata(&outputs[0]).unwrap().len(),
                fs::metadata(&outputs[1]).unwrap().len()
            );
            println!("{row}");
            writeln!(summary, "{row}").unwrap();
            raw.flush().unwrap();
            summary.flush().unwrap();
        }
    }
}
