// Match CLI contracts without freezing a particular compiler release number.
use std::path::Path;

use super::build;
use crate::oracle;

pub fn execute(compiler: &Path, args: &[String], status: i32, stdout: &str) -> Result<(), String> {
    let output = build::invoke(compiler, args)?;
    oracle::exit(&output, status)?;
    if output.stdout != stdout.as_bytes() {
        return Err(format!("unexpected information output: {output:?}"));
    }
    Ok(())
}

// Numeric components have no fixed release values or machine-integer size limit.
fn digits(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

// Validate the documented stable, snapshot and nightly output shapes independently.
fn valid_version(output: &[u8]) -> bool {
    let Ok(text) = std::str::from_utf8(output) else {
        return false;
    };
    let Some(line) = text.strip_suffix('\n') else {
        return false;
    };
    let stable = |value: &str| {
        value
            .split_once('.')
            .is_some_and(|(major, minor)| digits(major) && digits(minor))
    };
    if stable(line) {
        return true;
    }
    if let Some((base, snapshot)) = line.split_once("-sp") {
        return stable(base) && digits(snapshot);
    }
    let fields: Vec<_> = line.split('-').collect();
    if let [date, time, release, identifier] = fields.as_slice() {
        return date.len() == 8
            && digits(date)
            && time.split_once(':').is_some_and(|(hour, minute)| {
                hour.len() == 2 && minute.len() == 2 && digits(hour) && digits(minute)
            })
            && release.strip_prefix("LCC").is_some_and(stable)
            && !identifier.is_empty()
            && identifier
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/' | b'='));
    }
    false
}

// Check syntax first; alias/idempotence checks also agree with the canonical flag.
pub fn version(compiler: &Path, args: &[String]) -> Result<(), String> {
    let output = build::invoke(compiler, args)?;
    oracle::exit(&output, 0)?;
    if !output.stderr.is_empty() || !valid_version(&output.stdout) {
        return Err(format!("invalid version output: {output:?}"));
    }
    if args != ["-v"] {
        let canonical = build::invoke(compiler, &["-v".into()])?;
        oracle::exit(&canonical, 0)?;
        if !canonical.stderr.is_empty() || canonical.stdout != output.stdout {
            return Err(format!(
                "version options disagree: {output:?} / {canonical:?}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::valid_version;

    #[test]
    fn release_numbers_can_change_without_rebuilding_the_test_contract() {
        for value in [
            "0.9\n",
            "27.42\n",
            "15.3-sp97\n",
            "11112026-19:30-LCC27.42-MVhKNTBJNjJPQk1OTDI1Nzk=\n",
        ] {
            assert!(valid_version(value.as_bytes()), "{value:?}");
        }
    }

    #[test]
    fn malformed_or_duplicate_information_is_not_a_version() {
        for value in [
            "",
            "\n",
            "27.42",
            "27.42\n\n",
            "27.42\n27.42\n",
            "garbage\n",
            "garbage27.42\n",
            ".42\n",
            "27.\n",
            "27.42-sp\n",
            "27.42-spx\n",
            "27.42 unexpected\n",
            "11112026-19:30-LCC27.42-\n",
        ] {
            assert!(!valid_version(value.as_bytes()), "{value:?}");
        }
        assert!(!valid_version(&[0xff, b'\n']));
    }
}
