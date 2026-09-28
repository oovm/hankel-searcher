/// Search ordering strategies for checkpoint improvement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchStrategy {
    /// Walk consecutive indices from the saved cursor.
    Enumerate,
    /// Visit the same forward window, closest to the current best index first.
    Local,
    /// Visit the same forward window in a seed-stable pseudo-random order.
    Sample,
}

impl SearchStrategy {
    pub fn parse(input: &str) -> Result<Self, String> {
        match input {
            "enumerate" => Ok(Self::Enumerate),
            "local" => Ok(Self::Local),
            "sample" => Ok(Self::Sample),
            other => Err(format!("unsupported search strategy `{other}`")),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Enumerate => "enumerate",
            Self::Local => "local",
            Self::Sample => "sample",
        }
    }
}
