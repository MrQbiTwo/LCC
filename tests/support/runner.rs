// Bounded workers run independent cases and preserve stable report ordering.
use std::{
    sync::{Arc, Mutex},
    thread,
};

// Apply a case function across at most eight concurrent workers.
pub fn parallel<T, F>(cases: Vec<T>, workers: usize, execute: F) -> Vec<(usize, Result<(), String>)>
where
    T: Send + 'static,
    F: Fn(T) -> Result<(), String> + Send + Sync + 'static,
{
    let jobs = Arc::new(Mutex::new(cases.into_iter().enumerate()));
    let results = Arc::new(Mutex::new(Vec::new()));
    let execute = Arc::new(execute);
    thread::scope(|scope| {
        for _ in 0..workers.clamp(1, 8) {
            let jobs = Arc::clone(&jobs);
            let results = Arc::clone(&results);
            let execute = Arc::clone(&execute);
            scope.spawn(move || loop {
                let item = jobs.lock().unwrap().next();
                let Some((index, case)) = item else { break };
                let result = execute(case);
                results.lock().unwrap().push((index, result));
            });
        }
    });
    let mut output = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    output.sort_by_key(|entry| entry.0);
    output
}
