use std::ops::RangeInclusive;

/// A builder for creating validation rules.
#[derive(Default)]
pub struct Validator {
    pub length: Option<RangeInclusive<usize>>,
    pub pattern: Option<String>,
    pub range: Option<RangeInclusive<f64>>,
}

impl Validator {
    /// Creates a new `Validator` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use sanitizr::Validator;
    ///
    /// let validator = Validator::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the length validation rule.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum allowed length.
    /// * `max` - The maximum allowed length.
    ///
    /// # Examples
    ///
    /// ```
    /// use sanitizr::Validator;
    ///
    /// let validator = Validator::new().length(5, 10);
    /// ```
    #[must_use]
    pub fn length(mut self, min: usize, max: usize) -> Self {
        self.length = Some(min..=max);
        self
    }

    /// Sets the pattern validation rule.
    ///
    /// # Arguments
    ///
    /// * `regex` - The regular expression pattern to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use sanitizr::Validator;
    ///
    /// let validator = Validator::new().pattern(r"^\w+$");
    /// ```
    #[must_use]
    pub fn pattern(mut self, regex: &str) -> Self {
        self.pattern = Some(regex.to_string());
        self
    }

    /// Sets the range validation rule.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum allowed value.
    /// * `max` - The maximum allowed value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sanitizr::Validator;
    ///
    /// let validator = Validator::new().range(0.0, 100.0);
    /// ```
    #[must_use]
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.range = Some(min..=max);
        self
    }
}

/// A trait for types that can be validated.
pub trait Validatable {
    /// Validates the value against the given validator.
    fn validate(&self, validator: &Validator) -> Result<(), String>;
}