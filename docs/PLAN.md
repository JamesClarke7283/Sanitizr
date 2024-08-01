# Sanitizr
A general validation library for rust.

We use the builder syntax to write a validator and we apply that validator to the type by attaching the `.validate` function to every type we implement validation rules for.

We need to implement default validators for every primitive datatype, u8,i8,string, etc. As long as we have implemented our validation rules for that type, the `.validate` function will attach to that type. Later on, we can implement validation for structs by using `Derive` to make a `StructValidator`.

We re-export all stuff through the `lib.rs` so we can use `StructValidator` and `Validator` from the root, but they are defined in seperate rust files.

## Example usage for single type validation
```rust
let name = "James";
let my_validator = Validator::new()
    .length(5, 10)
    .pattern(r"^\w+$");

name.validate(&my_validator);
```

## Example usage for struct validation
```rust
use sanitizr::Validate;

#[derive(StructValidator)]
struct User {
    #[validate(length(min = 5, max = 10))]
    username: String,

    #[validate(range(min = 18, max = 120))]
    age: u32,

    #[validate(pattern = r"^\w+@\w+\.\w+$")]
    email: String,
}

fn main() {
    let user = User {
        username: "James".to_string(),
        age: 25,
        email: "james@example.com".to_string(),
    };

    match user.validate() {
        Ok(_) => println!("Validation passed"),
        Err(errors) => println!("Validation failed: {:?}", errors),
    }
}
```

## Project Structure

.
├── Cargo.lock
├── Cargo.toml
├── docs
│   ├── LICENSE.md
│   └── PLAN.md
├── LICENSE.md
├── README.md
└── src
    ├── lib.rs - Applies the Validator to all applicable types in the `types` folder so we can use `.validate` on it.
    ├── structures.rs - defines `StructValidator` and relevent functions to structures.
    ├── types
    │   ├── collection.rs - all collections like `vec`, `arc`, etc.
    │   ├── mod.rs
    │   ├── number.rs - All number types, we can further refine and add validators for `integer` and `float` types.
    │   └── string.rs - String datatypes.
    └── validator.rs - Contains the builder syntax pattern implementation for `Validator`.

## Best practices

Please write Docstrings, Doctests and unit tests(using builtin libraries) for the implementation, we expect this to a high standard.
Everything should be documented and tested.