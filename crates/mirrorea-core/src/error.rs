use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirroreaCoreError {
    message: String,
}

impl MirroreaCoreError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for MirroreaCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for MirroreaCoreError {}

pub(crate) fn require_non_empty(
    carrier: &str,
    field: &str,
    value: &str,
) -> Result<(), MirroreaCoreError> {
    if value.trim().is_empty() {
        return Err(MirroreaCoreError::new(format!(
            "{carrier} field `{field}` must not be blank"
        )));
    }
    Ok(())
}

pub(crate) fn require_non_empty_items(
    carrier: &str,
    field: &str,
    values: &[String],
) -> Result<(), MirroreaCoreError> {
    for value in values {
        require_non_empty(carrier, field, value)?;
    }
    Ok(())
}
