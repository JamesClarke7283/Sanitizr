use crate::validator::{Validatable, Validator};

/// Extension trait for number types to add validation capabilities.
pub trait NumberExt: Validatable {}

macro_rules! impl_validatable_for_number {
    ($($t:ty),*) => {
        $(
            impl Validatable for $t {
                fn validate(&self, validator: &Validator) -> Result<(), String> {
                    if let Some(range) = &validator.range {
                        let value = *self as f64;
                        if !range.contains(&value) {
                            return Err(format!(
                                "Value must be between {} and {}, but was {}",
                                range.start(),
                                range.end(),
                                value
                            ));
                        }
                    }
                    Ok(())
                }
            }

            impl NumberExt for $t {}
        )*
    };
}

impl_validatable_for_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::Validator;

    #[test]
    fn test_integer_range_validation() {
        let validator = Validator::new().range(0.0, 100.0);
        assert!(50_i32.validate(&validator).is_ok());
        assert!((-1_i32).validate(&validator).is_err());
        assert!(101_i32.validate(&validator).is_err());
    }

    #[test]
    fn test_float_range_validation() {
        let validator = Validator::new().range(-1.5, 1.5);
        assert!(0.5_f64.validate(&validator).is_ok());
        assert!((-2.0_f64).validate(&validator).is_err());
        assert!(2.0_f64.validate(&validator).is_err());
    }

    #[test]
    fn test_unsigned_range_validation() {
        let validator = Validator::new().range(10.0, 20.0);
        assert!(15_u32.validate(&validator).is_ok());
        assert!(5_u32.validate(&validator).is_err());
        assert!(25_u32.validate(&validator).is_err());
    }
}