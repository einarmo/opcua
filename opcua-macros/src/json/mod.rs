use gen::{generate_json_decode_impl, generate_json_encode_impl, parse_json_struct};
use proc_macro2::TokenStream;
use syn::DeriveInput;

mod gen;

pub fn derive_json_encode_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_data = parse_json_struct(input)?;
    generate_json_encode_impl(struct_data)
}

pub fn derive_json_decode_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_data = parse_json_struct(input)?;
    generate_json_decode_impl(struct_data)
}
