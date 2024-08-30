use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HtmlElement)]
pub fn derive_html_element(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl ::stunseed::html::HtmlElement for #name {
            fn get_dom_element_mut(&mut self) -> &mut ::stunseed::ast::DomElement {
                &mut self.inner
            }

            fn get_dom_element(&self) -> ::stunseed::ast::DomElement {
                self.inner.clone()
            }
        }
    };
    TokenStream::from(expanded)
}
