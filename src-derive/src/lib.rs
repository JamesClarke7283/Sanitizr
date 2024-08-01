use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Data, Fields, parse::Parse, parse::ParseStream, Expr};
use proc_macro2;

struct ValidationArgs {
    expr: Expr,
}

impl Parse for ValidationArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        Ok(ValidationArgs { expr })
    }
}

impl ToTokens for ValidationArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.expr.to_tokens(tokens);
    }
}

#[proc_macro_derive(StructValidator, attributes(validate))]
pub fn derive_struct_validator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => &fields.named,
                _ => panic!("StructValidator can only be derived for structs with named fields"),
            }
        },
        _ => panic!("StructValidator can only be derived for structs"),
    };

    let validations = fields.iter().filter_map(|field| {
        let field_name = &field.ident;
        field.attrs.iter().find_map(|attr| {
            if attr.path().is_ident("validate") {
                let args = attr.parse_args::<ValidationArgs>().unwrap();
                Some(quote! {
                    if let Err(e) = self.#field_name.validate(&Validator::new().#args) {
                        errors.push(format!("{}: {}", stringify!(#field_name), e));
                    }
                })
            } else {
                None
            }
        })
    });

    let expanded = quote! {
        impl Validate for #name {
            fn validate(&self) -> Result<(), Vec<String>> {
                let mut errors = Vec::new();
                #(#validations)*
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    TokenStream::from(expanded)
}