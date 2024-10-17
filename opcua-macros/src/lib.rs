mod events;
mod utils;
mod xml;

use events::{derive_event_field_inner, derive_event_inner};
use proc_macro::TokenStream;
use syn::parse_macro_input;
use xml::derive_from_xml_inner;

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

#[proc_macro_derive(FromXml, attributes(opcua))]
pub fn derive_from_xml(item: TokenStream) -> TokenStream {
    match derive_from_xml_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
