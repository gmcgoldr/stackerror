use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn derive_stack_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl #name {
            fn from_error(error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
                Self(Error::from_error(error))
            }
        }

        impl StackError for #name {
            fn stack_error(self, error: impl std::fmt::Display + Send + Sync + 'static) -> Self {
                Self(self.0.stack_error(error))
            }
        }

        impl ErrorCode<ErrorHandling> for #name {
            fn code(&self) -> Option<&ErrorHandling> {
                self.0.code()
            }

            fn with_code(self, code: Option<ErrorHandling>) -> Self {
                Self(self.0.with_code(code))
            }
        }

        impl ErrorUri for #name {
            fn uri(&self) -> Option<&str> {
                self.0.uri()
            }

            fn with_uri(self, uri: Option<String>) -> Self {
                Self(self.0.with_uri(uri))
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
