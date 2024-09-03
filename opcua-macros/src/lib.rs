mod events;
mod utils;

use events::{derive_event_field_inner, derive_event_inner};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Event, attributes(opcua))]
pub fn derive_event(item: TokenStream) -> TokenStream {
    match derive_event_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_derive(EventField, attributes(opcua))]
pub fn derive_event_field(item: TokenStream) -> TokenStream {
    match derive_event_field_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
