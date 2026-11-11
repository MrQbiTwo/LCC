// Child-process isolation, bounded capture, and timeout cleanup using std only.
use std::{
    ffi::OsStr,
    io::{Read, Write},
    os::unix::process::CommandExt,
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

// Captured completion includes timeout and truncation independently of exit code.
#[derive(Debug)]
pub struct Output {
    pub status: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub timed_out: bool,
    pub truncated: bool,
}

// Linux process-group cleanup requires reserving the leader PID until final reap.
extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
    fn waitid(kind: i32, id: u32, information: *mut std::ffi::c_void, flags: i32) -> i32;
}

// Observe exit without reaping; the Linux siginfo storage is 128 aligned bytes.
fn wait_exit(pid: u32) -> Result<(), String> {
    let mut information = [0u64; 16];
    loop {
        let result = unsafe { waitid(1, pid, information.as_mut_ptr().cast(), 4 | 0x01000000) };
        if result == 0 {
            return Ok(());
        }
        let error = std::io::Error::last_os_error();
        if error.kind() != std::io::ErrorKind::Interrupted {
            return Err(format!("observe exit: {error}"));
        }
    }
}

// Drain continuously even after the capture limit to prevent pipe deadlocks.
fn drain(mut pipe: impl Read) -> (Vec<u8>, bool) {
    let mut bytes = Vec::new();
    let mut block = [0u8; 8192];
    let mut truncated = false;
    loop {
        let count = match pipe.read(&mut block) {
            Ok(0) => break,
            Ok(n) => n,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        };
        let keep = count.min((1024 * 1024usize).saturating_sub(bytes.len()));
        bytes.extend_from_slice(&block[..keep]);
        truncated |= keep != count;
    }
    (bytes, truncated)
}

// Execute one child without a shell and reap its process group on a deadline.
pub fn invoke(
    program: impl AsRef<OsStr>,
    args: &[String],
    input: Vec<u8>,
    timeout: Duration,
) -> Result<Output, String> {
    let has_input = !input.is_empty();
    let mut child = Command::new(program)
        .args(args)
        .process_group(0)
        .stdin(if has_input {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn: {e}"))?;
    let pid = child.id() as i32;
    let started = Instant::now();
    let (sender, receiver) = mpsc::channel();
    let writer = child.stdin.take().map(|mut stdin| {
        let sender = sender.clone();
        thread::spawn(move || {
            let _ = stdin.write_all(&input);
            drop(stdin);
            let _ = sender.send(());
        })
    });
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let out_sender = sender.clone();
    let out_thread = thread::spawn(move || {
        let output = drain(stdout);
        let _ = out_sender.send(());
        output
    });
    let err_sender = sender.clone();
    let err_thread = thread::spawn(move || {
        let output = drain(stderr);
        let _ = err_sender.send(());
        output
    });
    // Exit and pipe completion share one deadline, including inherited child pipes.
    let waiter = thread::spawn(move || {
        let result = wait_exit(pid as u32);
        let _ = sender.send(());
        result
    });
    let mut timed_out = false;
    for _ in 0..3 + usize::from(has_input) {
        match receiver.recv_timeout(timeout.saturating_sub(started.elapsed())) {
            Ok(()) => {}
            Err(mpsc::RecvTimeoutError::Timeout) => {
                timed_out = true;
                break;
            }
            Err(_) => break,
        }
    }
    // WNOWAIT keeps this identity reserved even if the leader already exited.
    // Clean up silent background descendants too, then release the leader PID.
    unsafe {
        kill(-pid, 9);
    }
    let wait_result = waiter.join().map_err(|_| "wait worker panicked");
    let status = child.wait().map_err(|e| format!("reap: {e}"));
    if let Some(writer) = writer {
        writer.join().map_err(|_| "stdin worker panicked")?;
    }
    let (stdout, out_truncated) = out_thread.join().map_err(|_| "stdout worker panicked")?;
    let (stderr, err_truncated) = err_thread.join().map_err(|_| "stderr worker panicked")?;
    wait_result??;
    Ok(Output {
        status: status?.code(),
        stdout,
        stderr,
        timed_out,
        truncated: out_truncated || err_truncated,
    })
}
