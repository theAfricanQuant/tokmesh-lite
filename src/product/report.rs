use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ValidationReport {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationIssue {
    pub code: String,
    pub location: String,
    pub message: String,
}

impl ValidationReport {
    #[must_use]
    pub fn new() -> Self {
        Self {
            valid: true,
            issues: Vec::new(),
        }
    }

    pub fn error(
        &mut self,
        code: impl Into<String>,
        location: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.valid = false;
        self.issues.push(ValidationIssue {
            code: code.into(),
            location: location.into(),
            message: message.into(),
        });
    }

    pub fn append(&mut self, mut other: Self) {
        self.valid &= other.valid;
        self.issues.append(&mut other.issues);
    }
}
