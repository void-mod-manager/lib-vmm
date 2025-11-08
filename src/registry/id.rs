use crate::registry::RegistryError;

// Normalization rules
//  - lowercase
//  - one optional colon seperating namespace and rest
//  - allowed chars: a-z 0-9 . _ -
//  - 1..200 length
/// Helper function to normalize IDs before usage.
pub fn normalize_id(raw: &str) -> Result<String, RegistryError> {
    let s = raw.trim().to_lowercase();
    if s.is_empty() || s.len() > 200 {
        return Err(RegistryError::InvalidId(format!("ID '{}' is invalid.", raw)));
    }

    let mut seen_colon = false;
    for (i, ch) in s.chars().enumerate() {
        match ch {
            'a'..='z' | '0'..='9' | '.' | '_' | '-' => {}
            ':' if !seen_colon && i > 0 && i < s.len() - 1 => {
                seen_colon = true;
            }
            _ => {
                return Err(RegistryError::InvalidId(
                    format!("ID: '{}' contains invalid character '{}' at position {}", raw, ch, i)
                ))
            }
        }
    }
    Ok(s)
}

/// Helper function to check if an ID belongs to core or not
pub fn is_core_id(id: &str) -> bool {
    id.starts_with("core:")
}
