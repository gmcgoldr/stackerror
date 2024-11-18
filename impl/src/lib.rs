use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn derive_stack_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let first_field_type = if let syn::Data::Struct(data) = &input.data {
        if let Some(field) = data.fields.iter().next() {
            &field.ty
        } else {
            panic!("Expected at least one field");
        }
    } else {
        panic!("Expected a struct");
    };

    let expanded = quote! {
        #input

        impl #name {
            fn new(error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
                Self(#first_field_type::new(error))
            }
        }

        impl ErrorStacks for #name {
            fn stack_err(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
                Self(self.0.stack_err(error))
            }
        }

        impl ErrorWithCode<ErrorCode> for #name {
            fn err_code(&self) -> Option<&ErrorCode> {
                self.0.err_code()
            }

            fn with_err_code(self, code: Option<ErrorCode>) -> Self {
                Self(self.0.with_err_code(code))
            }
        }

        impl ErrorWithUri for #name {
            fn err_uri(&self) -> Option<&str> {
                self.0.err_uri()
            }

            fn with_err_uri(self, uri: Option<String>) -> Self {
                Self(self.0.with_err_uri(uri))
            }
        }

        impl ErrorStacksWithCode for #name {
            fn stack_err_code(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
                Self(self.0.stack_err_code(error))
            }
        }

        impl ErrorStacksWithUri for #name {
            fn stack_err_uri(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
                Self(self.0.stack_err_uri(error))
            }
        }

        impl ErrorStacksWithCodeUri for #name {
            fn stack_err_code_uri(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
                Self(self.0.stack_err_code_uri(error))
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::error::Error for #name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.0.source()
            }
        }
    };

    TokenStream::from(expanded)
}
