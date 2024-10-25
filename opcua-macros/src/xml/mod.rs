use gen::{generate_xml_impl, parse_xml_struct};
use proc_macro2::TokenStream;
use syn::DeriveInput;

mod gen;

pub fn derive_from_xml_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_data = parse_xml_struct(input)?;
    generate_xml_impl(struct_data)
}
