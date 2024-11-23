use validator::ValidationError;

pub fn non_empty(s: &str) -> Result<(), ValidationError> {
    if s.is_empty() {
        return Err(ValidationError::new("must be non-empty"));
    }
    Ok(())
}

pub fn each_non_empty(v: &Vec<String>) -> Result<(), ValidationError> {
    for s in v {
        if s.is_empty() {
            return Err(ValidationError::new("each element must be non-empty"));
        }
    }
    Ok(())
}
