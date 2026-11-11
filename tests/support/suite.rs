// Select validated case data, then run it without knowing individual groups.
use std::{
    collections::HashSet,
    fs,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use crate::{
    catalog::{self, Group, Suite},
    cli::Options,
    execution, runner, Case,
};

#[cfg(test)]
#[path = "suite_tests.rs"]
mod tests;

pub fn select(groups: &[Group], options: &Options) -> Result<Vec<Case>, String> {
    for name in &options.groups {
        if !groups
            .iter()
            .any(|group| group.suite == options.suite && group.name == name)
        {
            return Err(format!(
                "unknown group in {:?} suite: {name}",
                options.suite
            ));
        }
    }
    let mut selected = Vec::new();
    let mut ids = HashSet::new();
    for group in groups.iter().filter(|group| group.suite == options.suite) {
        for case in (group.cases)() {
            validate(&case)?;
            if !ids.insert(case.id.clone()) {
                return Err(format!("duplicate case ID: {}", case.id));
            }
            let group_matches =
                options.groups.is_empty() || options.groups.iter().any(|name| name == group.name);
            let id_matches = options
                .filter
                .as_ref()
                .map_or(true, |filter| case.id.contains(filter));
            if group_matches && id_matches {
                selected.push(case);
            }
        }
    }
    if selected.is_empty() {
        return Err("no matching tests".into());
    }
    Ok(selected)
}

fn validate(case: &Case) -> Result<(), String> {
    if case.id.is_empty()
        || !case
            .id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        return Err(format!("invalid case ID: {:?}", case.id));
    }
    if case.purpose.trim().is_empty() {
        return Err(format!("missing purpose: {}", case.id));
    }
    let mut files = HashSet::new();
    for (name, _) in &case.files {
        if name.is_empty() || *name == "." || *name == ".." || name.contains(['/', '\\', ':']) {
            return Err(format!(
                "invalid auxiliary filename in {}: {name:?}",
                case.id
            ));
        }
        if *name == "input.c" || !files.insert(*name) {
            return Err(format!(
                "duplicate/reserved auxiliary filename in {}: {name}",
                case.id
            ));
        }
    }
    Ok(())
}

// Return a process status; errors never masquerade as an empty passing suite.
pub fn run(options: Options) -> Result<i32, String> {
    if options.help {
        println!("{}", crate::cli::HELP);
        return Ok(0);
    }
    let cases = select(catalog::GROUPS, &options)?;
    if options.list_groups {
        for group in catalog::GROUPS
            .iter()
            .filter(|group| group.suite == options.suite)
        {
            if !options.groups.is_empty() && !options.groups.iter().any(|name| name == group.name) {
                continue;
            }
            let count = (group.cases)()
                .iter()
                .filter(|case| {
                    options
                        .filter
                        .as_ref()
                        .map_or(true, |filter| case.id.contains(filter))
                })
                .count();
            if count != 0 {
                println!("{}: {count} cases", group.name);
            }
        }
        return Ok(0);
    }
    if options.list {
        for case in cases {
            println!("{}: {}", case.id, case.purpose);
        }
        return Ok(0);
    }
    let compiler = options
        .compiler
        .ok_or("a compiler path is required to run cases")?
        .canonicalize()
        .map_err(|error| format!("compiler path: {error}"))?;
    let parent = compiler.parent().ok_or("compiler path has no parent")?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let root = parent.join(format!("test_work_{}_{stamp}", std::process::id()));
    fs::create_dir(&root).map_err(|error| format!("create isolated run directory: {error}"))?;
    let retained = root.clone();
    let ids: Vec<_> = cases.iter().map(|case| case.id.clone()).collect();
    let started = Instant::now();
    let results = runner::parallel(cases, options.jobs, move |case| {
        execution::execute(case, &compiler, &root)
    });
    let mut failures = 0;
    for (index, result) in results {
        if let Err(error) = result {
            failures += 1;
            eprintln!("FAIL {}: {error}", ids[index]);
        }
    }
    let repetitions = match options.suite {
        Suite::Ordinary => "native and NASM modes share case IDs",
        Suite::Interop => "GCC/Clang at -O0/-O2 repeat case IDs",
    };
    println!(
        "{}/{} cases passed in {:.3}s; {repetitions}",
        ids.len() - failures,
        ids.len(),
        started.elapsed().as_secs_f64()
    );
    if failures != 0 {
        eprintln!("Failure artifacts: {}", retained.display());
        return Ok(1);
    }
    fs::remove_dir(retained).map_err(|error| format!("remove empty run directory: {error}"))?;
    Ok(0)
}
