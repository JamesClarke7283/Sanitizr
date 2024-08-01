use crate::validator::{Validatable, Validator};
use regex::Regex;
use std::collections::HashMap;

/// Extension trait for String types to add validation capabilities.
pub trait StringExt: Validatable {}

impl<T: AsRef<str>> Validatable for T
where
    T: ?Sized,
    HashMap<T, T>: Validatable, // This ensures we don't implement for HashMap
{
    fn validate(&self, validator: &Validator) -> Result<(), String> {
        let value = self.as_ref();

        if let Some(length_range) = &validator.length {
            let length = value.len();
            if !length_range.contains(&length) {
                return Err(format!(
                    "Length must be between {} and {}, but was {}",
                    length_range.start(),
                    length_range.end(),
                    length
                ));
            }
        }

        if let Some(pattern) = &validator.pattern {
            let regex = Regex::new(pattern).map_err(|e| format!("Invalid regex pattern: {}", e))?;
            if !regex.is_match(value) {
                return Err(format!("Value does not match the pattern: {}", pattern));
            }
        }

        Ok(())
    }
}

impl<T: AsRef<str> + ?Sized> StringExt for T where HashMap<T, T>: Validatable {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::Validator;

    #[test]
    fn test_string_length_validation() {
        let validator = Validator::new().length(5, 10);
        assert!("hello".validate(&validator).is_ok());
        assert!("hi".validate(&validator).is_err());
        assert!("hello world".validate(&validator).is_err());
    }

    #[test]
    fn test_string_pattern_validation() {
        let validator = Validator::new().pattern(r"^\d{3}-\d{3}-\d{4}$");
        assert!("123-456-7890".validate(&validator).is_ok());
        assert!("123-45-67890".validate(&validator).is_err());
    }

    #[test]
    fn test_string_combined_validation() {
        let validator = Validator::new().length(5, 15).pattern(r"^[a-zA-Z]+$");
        assert!("HelloWorld".validate(&validator).is_ok());
        assert!("Hi".validate(&validator).is_err());
        assert!("Hello123World".validate(&validator).is_err());
    }
}