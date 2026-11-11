// Shared data contract: groups describe cases; the harness owns execution.
#[derive(Clone, Debug)]
pub enum Expectation {
    Exit(i32),
    Decimal(String, u32, u64, u64, i32),
    AssemblyHarness(String),
    CInterop(String),
    Io(String, String, i32),
    IoTimeout(String, String),
    Reject(String),
    Cli(Vec<String>, i32, String),
    CliVersion(Vec<String>),
}

#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub purpose: String,
    pub source: String,
    pub files: Vec<(&'static str, &'static str)>,
    pub expectation: Expectation,
}

impl Case {
    // Use one field order in every group: identity, purpose, input, expectation.
    pub fn new(
        id: impl Into<String>,
        purpose: impl Into<String>,
        source: impl Into<String>,
        expectation: Expectation,
    ) -> Self {
        Self {
            id: id.into(),
            purpose: purpose.into(),
            source: source.into(),
            files: Vec::new(),
            expectation,
        }
    }

    // Auxiliary files are local to the isolated directory of this case.
    pub fn with_files(mut self, files: Vec<(&'static str, &'static str)>) -> Self {
        self.files = files;
        self
    }
}
