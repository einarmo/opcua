mod events;
#[cfg(feature = "json")]
mod json;
mod utils;
#[cfg(feature = "xml")]
mod xml;

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

#[cfg(feature = "xml")]
#[proc_macro_derive(FromXml, attributes(opcua))]
pub fn derive_from_xml(item: TokenStream) -> TokenStream {
    use xml::derive_from_xml_inner;

    match derive_from_xml_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(feature = "json")]
#[proc_macro_derive(JsonEncodable, attributes(opcua))]
pub fn derive_json_encodable(item: TokenStream) -> TokenStream {
    use json::derive_json_encode_inner;

    match derive_json_encode_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(feature = "json")]
#[proc_macro_derive(JsonDecodable, attributes(opcua))]
pub fn derive_json_decodable(item: TokenStream) -> TokenStream {
    use json::derive_json_decode_inner;

    match derive_json_decode_inner(parse_macro_input!(item)) {
        Ok(r) => r.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
