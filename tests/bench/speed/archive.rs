// Persist inputs and evidence without copying filesystem-specific file metadata.
use crate::{
    cases::Case,
    methods::{checked, Method},
};
use std::{
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct Archive {
    pub work: PathBuf,
    pub path: PathBuf,
    pub samples: fs::File,
    pub summary: fs::File,
}

pub fn csv(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

// A content fingerprint identifies inputs/artifacts; it is not a security hash.
pub fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325u64, |value, byte| {
        (value ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

pub fn copy_contents(source: &Path, destination: &Path, executable: bool) -> Result<(), String> {
    let bytes = fs::read(source).map_err(|e| format!("read {}: {e}", source.display()))?;
    fs::write(destination, bytes).map_err(|e| format!("write {}: {e}", destination.display()))?;
    if executable {
        fs::set_permissions(destination, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("set executable permission: {e}"))?;
    }
    Ok(())
}

impl Archive {
    // Both directories are exclusive, and failed runs are retained for inspection.
    pub fn create(work: &str, archive: &str) -> Result<Self, String> {
        let work = PathBuf::from(work);
        let path = PathBuf::from(archive);
        if work.exists() || path.exists() {
            return Err("work and archive directories must both be new".into());
        }
        fs::create_dir(&work).map_err(|e| format!("create work directory: {e}"))?;
        fs::create_dir(&path).map_err(|e| format!("create archive directory: {e}"))?;
        let work = fs::canonicalize(work).map_err(|e| e.to_string())?;
        let path = fs::canonicalize(path).map_err(|e| e.to_string())?;
        fs::create_dir(path.join("inputs")).map_err(|e| e.to_string())?;
        let mut samples = fs::File::create(path.join("samples.csv")).map_err(|e| e.to_string())?;
        let mut summary = fs::File::create(path.join("summary.csv")).map_err(|e| e.to_string())?;
        writeln!(
            samples,
            "case,family,source_bytes,method,trial,wall_ms,output_bytes,output_fnv1a64"
        )
        .map_err(|e| e.to_string())?;
        writeln!(summary, "case,family,source_bytes,source_lines,method,runs,median_ms,p95_ms,min_ms,max_ms,output_bytes,mib_per_second")
            .map_err(|e| e.to_string())?;
        Ok(Self {
            work,
            path,
            samples,
            summary,
        })
    }

    pub fn inputs(&self, cases: &[Case]) -> Result<(), String> {
        let mut catalog =
            fs::File::create(self.path.join("cases.csv")).map_err(|e| e.to_string())?;
        writeln!(
            catalog,
            "case,family,purpose,expected_exit,source_bytes,source_fnv1a64"
        )
        .map_err(|e| e.to_string())?;
        for case in cases {
            fs::write(
                self.path.join("inputs").join(format!("{}.c", case.id)),
                case.source,
            )
            .map_err(|e| e.to_string())?;
            writeln!(
                catalog,
                "{},{},{},{},{},{:016x}",
                csv(case.id),
                csv(case.family),
                csv(case.purpose),
                case.expected,
                case.source.len(),
                hash(case.source.as_bytes())
            )
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn metadata(
        &self,
        methods: &[Method],
        trials: usize,
        filter: &str,
        args: &[String],
    ) -> Result<(), String> {
        let mut file =
            fs::File::create(self.path.join("metadata.txt")).map_err(|e| e.to_string())?;
        writeln!(file, "unix_time={}\narguments={args:?}\nwork={}\narchive={}\nfilter={filter:?}\nwarmups=1\ntrials={trials}",
            SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs(),
            self.work.display(), self.path.display()).map_err(|e| e.to_string())?;
        writeln!(file, "measurement=serial compiler process wall time in milliseconds\nprocess_overhead=bounded capture, wait threads, process-group cleanup included identically\nverification=first and changed artifacts independently assembled/linked/executed outside timing; later exact-byte-identical artifacts reuse prior verification\nverification_reuse=full byte-for-byte comparison per case/method, never hash-only; output reads/comparisons/hashes are untimed\norder=rotate and alternate direction by case and round\nstatistics=median; nearest-rank p95; one warmup excluded\nelf_scope=LCC emits a static ELF directly; GCC/Clang driver also invokes the system linker and C startup/runtime\nasm_scope=compiler assembly output only; LCC NASM, GCC/Clang GAS\nlatest_versions=verify current stable upstream releases separately; exact selected tool versions follow\nfingerprints=FNV-1a 64-bit content identity, not cryptographic verification\nstdout_and_stderr=generated executables must produce neither")
            .map_err(|e| e.to_string())?;
        let lcc = fs::read(&methods[0].program).map_err(|e| e.to_string())?;
        writeln!(
            file,
            "lcc_bytes={}\nlcc_fnv1a64={:016x}",
            lcc.len(),
            hash(&lcc)
        )
        .map_err(|e| e.to_string())?;
        let version = checked(&methods[0].program, &["-v".into()], Duration::from_secs(10))?;
        writeln!(
            file,
            "lcc_version={}",
            String::from_utf8_lossy(&version.stdout).trim_end()
        )
        .map_err(|e| e.to_string())?;
        for method in methods {
            writeln!(
                file,
                "method={} program={:?} flags={:?}",
                method.name, method.program, method.flags
            )
            .map_err(|e| e.to_string())?;
        }
        for (name, program) in [
            ("gcc", methods[1].program.as_path()),
            ("clang", methods[2].program.as_path()),
            ("nasm", Path::new("nasm")),
            ("ld", Path::new("ld")),
        ] {
            let result = checked(program, &["--version".into()], Duration::from_secs(10))?;
            writeln!(
                file,
                "\n[{name} version]\n{}{}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            )
            .map_err(|e| e.to_string())?;
        }
        let system = checked(Path::new("uname"), &["-a".into()], Duration::from_secs(10))?;
        writeln!(
            file,
            "\n[system]\n{}",
            String::from_utf8_lossy(&system.stdout)
        )
        .map_err(|e| e.to_string())?;
        for (name, prefix) in [
            ("/proc/cpuinfo", "model name"),
            ("/proc/meminfo", "MemTotal:"),
        ] {
            if let Ok(value) = fs::read_to_string(name) {
                if let Some(line) = value.lines().find(|line| line.starts_with(prefix)) {
                    writeln!(file, "{line}").map_err(|e| e.to_string())?;
                }
            }
        }
        file.flush().map_err(|e| e.to_string())
    }

    // Preserve current generated files even if the work directory is temporary.
    pub fn failure(&self, case: &str, error: &str) -> Result<(), String> {
        let destination = self.path.join("failure");
        fs::create_dir(&destination).map_err(|e| e.to_string())?;
        fs::write(
            destination.join("diagnostics.txt"),
            format!("case={case}\n{error}\n"),
        )
        .map_err(|e| e.to_string())?;
        let source = self.work.join(case);
        if source.is_dir() {
            for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                if entry.file_type().map_err(|e| e.to_string())?.is_file() {
                    copy_contents(&entry.path(), &destination.join(entry.file_name()), false)?;
                }
            }
        }
        Ok(())
    }
}
