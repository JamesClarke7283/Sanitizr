/// A trait for types that can be validated as a whole.
pub trait Validate {
    /// Validates the entire structure.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the structure passes all validation rules, or an `Err` with a vector of error messages.
    fn validate(&self) -> Result<(), Vec<String>>;
}