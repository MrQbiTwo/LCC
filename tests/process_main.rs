// Exercise bounded process capture and cleanup independently of compiler cases.
#[path = "support/process.rs"]
mod process;
use std::{
    fs,
    time::{Duration, Instant},
};

// Static shell fixtures deliberately exercise child and descendant lifetimes.
fn shell(script: &str, input: Vec<u8>, timeout: Duration) -> process::Output {
    process::invoke("/bin/sh", &["-c".into(), script.into()], input, timeout).unwrap()
}

#[test]
fn status_and_streams() {
    let output = shell(
        "printf out; printf err >&2; exit 7",
        vec![],
        Duration::from_secs(2),
    );
    assert_eq!(output.status, Some(7));
    assert_eq!(output.stdout, b"out");
    assert_eq!(output.stderr, b"err");
    assert!(!output.timed_out && !output.truncated);
}

#[test]
fn empty_input_is_eof() {
    let output = process::invoke("/bin/cat", &[], vec![], Duration::from_secs(2)).unwrap();
    assert_eq!(output.status, Some(0));
    assert!(output.stdout.is_empty() && !output.timed_out);
}

#[test]
fn binary_input_round_trip() {
    let input: Vec<u8> = (0..65536).map(|i| (i % 256) as u8).collect();
    let output = process::invoke("/bin/cat", &[], input.clone(), Duration::from_secs(2)).unwrap();
    assert_eq!(output.status, Some(0));
    assert_eq!(output.stdout, input);
    assert!(!output.timed_out && !output.truncated);
}

#[test]
fn truncated_output_still_drains() {
    let output = process::invoke(
        "/bin/cat",
        &[],
        vec![b'x'; 2 * 1024 * 1024],
        Duration::from_secs(2),
    )
    .unwrap();
    assert_eq!(output.status, Some(0));
    assert_eq!(output.stdout.len(), 1024 * 1024);
    assert!(output.truncated && !output.timed_out);
}

#[test]
fn timeout_kills_the_process_group() {
    let started = Instant::now();
    let output = shell("sleep 10 & wait", vec![], Duration::from_millis(100));
    assert!(output.timed_out && output.status.is_none());
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn inherited_pipes_share_the_deadline() {
    let started = Instant::now();
    let output = shell("sleep 10 & exit 0", vec![], Duration::from_millis(100));
    assert!(output.timed_out);
    assert_eq!(output.status, Some(0));
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn early_exit_does_not_block_input_writer() {
    let output = shell(
        "exit 0",
        vec![b'x'; 2 * 1024 * 1024],
        Duration::from_secs(2),
    );
    assert_eq!(output.status, Some(0));
    assert!(!output.timed_out);
}

#[test]
fn direct_children_are_reaped() {
    let output = shell("printf '%s' $$", vec![], Duration::from_secs(2));
    let pid = String::from_utf8(output.stdout).unwrap();
    assert!(!fs::exists(format!("/proc/{pid}")).unwrap());
}

#[test]
fn spawn_failure_is_reported() {
    let error = process::invoke(
        "/lcc_missing_test_program",
        &[],
        vec![],
        Duration::from_secs(2),
    )
    .unwrap_err();
    assert!(error.starts_with("spawn:"));
}
