// Compare cached and forced compiler builds in a new, retained output directory.
use std::{env, fs, path::Path, process::Command, time::Instant};

fn build(directory: &Path) -> f64 {
    let start = Instant::now();
    let output = Command::new("make")
        .args(["all", &format!("BUILD={}", directory.display())])
        .output()
        .expect("run make");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    start.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "build_speed <new_build_directory>");
    let directory = Path::new(&args[1]);
    fs::create_dir(directory).expect("new output directory");
    build(directory);
    let runtime_time = fs::metadata(directory.join("runtime_pack.inc"))
        .unwrap()
        .modified()
        .unwrap();
    let mut rows = String::from("round,cached_ms,forced_ms\n");
    let mut samples = [Vec::new(), Vec::new()];
    for round in 0..5 {
        let mut times = [0.0; 2];
        for offset in 0..2 {
            let which = (round + offset) % 2;
            if which == 1 {
                // Invalidate only the stamp in this probe's newly created directory.
                fs::remove_file(directory.join("compiler.stamp")).unwrap();
            }
            let previous = fs::metadata(directory.join("lcc"))
                .unwrap()
                .modified()
                .unwrap();
            times[which] = build(directory);
            let current = fs::metadata(directory.join("lcc"))
                .unwrap()
                .modified()
                .unwrap();
            if which == 0 {
                assert_eq!(previous, current, "cache rebuilt the compiler");
            }
            assert_eq!(
                runtime_time,
                fs::metadata(directory.join("runtime_pack.inc"))
                    .unwrap()
                    .modified()
                    .unwrap()
            );
            samples[which].push(times[which]);
        }
        rows.push_str(&format!("{round},{:.3},{:.3}\n", times[0], times[1]));
        fs::write(directory.join("samples.csv"), &rows).unwrap();
    }
    for sample in &mut samples {
        sample.sort_by(f64::total_cmp);
    }
    println!(
        "cached_median_ms={:.3}, forced_median_ms={:.3}, speedup={:.3}",
        samples[0][2],
        samples[1][2],
        samples[1][2] / samples[0][2]
    );
}
