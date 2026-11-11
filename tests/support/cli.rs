// Command-line policy is independent of the registered groups and case kinds.
use std::path::PathBuf;

use crate::catalog::Suite;

pub const HELP: &str = "Usage: tests_main [compiler] [filter] [options]

  --list          List selected case IDs and purposes; no compiler needed
  --groups        List available groups in the selected suite
  --interop       Select the optional GCC/Clang suite
  --group NAME    Select a group; may be repeated
  --filter TEXT   Select IDs containing TEXT (also accepted positionally)
  --jobs N        Run 1 to 8 workers (default: 4)
  --help          Show this usage

Examples:
  tests_main build/lcc
  tests_main build/lcc array_bounds
  tests_main --list --group tentative
  tests_main build/lcc --interop --group storage_interop --jobs 2";

#[derive(Debug, PartialEq, Eq)]
pub struct Options {
    pub compiler: Option<PathBuf>,
    pub suite: Suite,
    pub groups: Vec<String>,
    pub filter: Option<String>,
    pub jobs: usize,
    pub list: bool,
    pub list_groups: bool,
    pub help: bool,
}

impl Options {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut options = Self {
            compiler: None,
            suite: Suite::Ordinary,
            groups: Vec::new(),
            filter: None,
            jobs: 4,
            list: false,
            list_groups: false,
            help: false,
        };
        let mut args = args.into_iter();
        let mut positional_only = false;
        while let Some(arg) = args.next() {
            if !positional_only {
                match arg.as_str() {
                    "--" => {
                        positional_only = true;
                        continue;
                    }
                    "--help" => {
                        options.help = true;
                        continue;
                    }
                    "--list" => {
                        options.list = true;
                        continue;
                    }
                    "--groups" => {
                        options.list_groups = true;
                        continue;
                    }
                    "--interop" => {
                        options.suite = Suite::Interop;
                        continue;
                    }
                    "--group" => {
                        options.groups.push(value(&mut args, &arg)?);
                        continue;
                    }
                    "--filter" => {
                        if options.filter.is_some() {
                            return Err("filter specified more than once".into());
                        }
                        options.filter = Some(value(&mut args, &arg)?);
                        continue;
                    }
                    "--jobs" => {
                        options.jobs = value(&mut args, &arg)?
                            .parse()
                            .ok()
                            .filter(|jobs| (1..=8).contains(jobs))
                            .ok_or("--jobs requires an integer from 1 to 8")?;
                        continue;
                    }
                    _ if arg.starts_with('-') => return Err(format!("unknown option: {arg}")),
                    _ => {}
                }
            }
            if options.compiler.is_none() {
                options.compiler = Some(arg.into());
            } else if options.filter.is_none() {
                options.filter = Some(arg);
            } else {
                return Err(format!("unexpected argument: {arg}"));
            }
        }
        if options.list && options.list_groups {
            return Err("choose --list or --groups".into());
        }
        if options.compiler.is_none() && !options.list && !options.list_groups && !options.help {
            return Err("a compiler path is required to run cases".into());
        }
        Ok(options)
    }
}

fn value(args: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    args.next()
        .filter(|value| !value.is_empty() && !value.starts_with("--"))
        .ok_or_else(|| format!("{option} requires a value"))
}
