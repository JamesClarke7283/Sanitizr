use crate::validator::{Validatable, Validator};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

/// Extension trait for collection types to add validation capabilities.
pub trait CollectionExt: Validatable {}

macro_rules! impl_validatable_for_collection {
    ($($t:ty),*) => {
        $(
            impl<T: Validatable> Validatable for $t {
                fn validate(&self, validator: &Validator) -> Result<(), String> {
                    if let Some(length_range) = &validator.length {
                        let length = self.len();
                        if !length_range.contains(&length) {
                            return Err(format!(
                                "Collection length must be between {} and {}, but was {}",
                                length_range.start(),
                                length_range.end(),
                                length
                            ));
                        }
                    }

                    for (index, item) in self.iter().enumerate() {
                        if let Err(e) = item.validate(validator) {
                            return Err(format!("Invalid item at index {}: {}", index, e));
                        }
                    }

                    Ok(())
                }
            }

            impl<T: Validatable> CollectionExt for $t {}
        )*
    };
}

impl_validatable_for_collection!(Vec<T>, VecDeque<T>, HashSet<T>);

impl<K: Validatable + Debug, V: Validatable> Validatable for HashMap<K, V> {
    fn validate(&self, validator: &Validator) -> Result<(), String> {
        if let Some(length_range) = &validator.length {
            let length = self.len();
            if !length_range.contains(&length) {
                return Err(format!(
                    "HashMap length must be between {} and {}, but was {}",
                    length_range.start(),
                    length_range.end(),
                    length
                ));
            }
        }

        for (key, value) in self {
            if let Err(e) = key.validate(validator) {
                return Err(format!("Invalid key {:?}: {}", key, e));
            }
            if let Err(e) = value.validate(validator) {
                return Err(format!("Invalid value for key {:?}: {}", key, e));
            }
        }

        Ok(())
    }
}

impl<K: Validatable + Debug, V: Validatable> CollectionExt for HashMap<K, V> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::Validator;

    #[test]
    fn test_vec_length_validation() {
        let validator = Validator::new().length(2, 4);
        assert!(vec![1, 2, 3].validate(&validator).is_ok());
        assert!(vec![1].validate(&validator).is_err());
        assert!(vec![1, 2, 3, 4, 5].validate(&validator).is_err());
    }

    #[test]
    fn test_vec_item_validation() {
        let validator = Validator::new().range(0.0, 10.0);
        assert!(vec![1, 5, 9].validate(&validator).is_ok());
        assert!(vec![1, 5, 11].validate(&validator).is_err());
    }

    #[test]
    fn test_hashmap_validation() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 5);
        map.insert("key2".to_string(), 8);

        let validator = Validator::new()
            .length(1, 3)
            .pattern(r"^key\d$")
            .range(0.0, 10.0);

        assert!(map.validate(&validator).is_ok());

        map.insert("invalid_key".to_string(), 15);
        assert!(map.validate(&validator).is_err());
    }
}