// Stable entry point; case registration belongs only in catalog.rs.
#[path = "support/case.rs"]
mod case;
mod catalog;
#[path = "support/cli.rs"]
mod cli;
#[path = "support/execute.rs"]
mod execution;
#[path = "support/interop.rs"]
mod interop_runner;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/process.rs"]
mod process;
#[path = "support/runner.rs"]
mod runner;
#[path = "support/stdio.rs"]
mod stdio_runner;
#[path = "support/suite.rs"]
mod suite;

pub use case::{Case, Expectation};
pub use execution::invoke;

fn main() {
    let result = cli::Options::parse(std::env::args().skip(1)).and_then(suite::run);
    let status = match result {
        Ok(status) => status,
        Err(error) => {
            eprintln!("tests_main: {error}\nUse --help for usage.");
            2
        }
    };
    std::process::exit(status);
}
