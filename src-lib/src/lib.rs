pub mod types;
pub mod structures;
pub mod validator;

pub use structures::Validate;
pub use validator::{Validator, Validatable};
pub use sanitizr_derive::StructValidator;

// Re-export types that implement Validatable
pub use types::string::StringExt;
pub use types::number::NumberExt;
pub use types::collection::CollectionExt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_validation() {
        let validator = Validator::new().length(5, 10);
        assert!("hello".validate(&validator).is_ok());
        assert!("hi".validate(&validator).is_err());
    }

    #[test]
    fn test_number_validation() {
        let validator = Validator::new().range(0.0, 100.0);
        assert!(50.0.validate(&validator).is_ok());
        assert!((-1.0).validate(&validator).is_err());
    }

    #[test]
    fn test_struct_validation() {
        #[derive(StructValidator)]
        struct User {
            #[validate(length(5, 10))]
            username: String,

            #[validate(range(18.0, 120.0))]
            age: u32,
        }

        impl Validatable for User {
            fn validate(&self, _: &Validator) -> Result<(), String> {
                // This method is required to satisfy the Validatable trait
                // The actual validation is done by the derived Validate implementation
                Ok(())
            }
        }

        let user = User {
            username: "James".to_string(),
            age: 25,
        };

        let validator = Validator::new();
        assert!(Validate::validate(&user).is_ok());
        assert!(Validatable::validate(&user, &validator).is_ok());
    }
}