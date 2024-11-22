use gen::{generate_binary_decode_impl, generate_binary_encode_impl, parse_binary_struct};
use proc_macro2::TokenStream;
use syn::DeriveInput;

mod gen;
pub fn derive_binary_encode_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_data = parse_binary_struct(input)?;
    generate_binary_encode_impl(struct_data)
}

pub fn derive_binary_decode_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_data = parse_binary_struct(input)?;
    generate_binary_decode_impl(struct_data)
}
