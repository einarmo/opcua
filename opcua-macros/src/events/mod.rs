mod gen;
mod parse;

use gen::generate_event_impls;
use parse::parse_event_struct;
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn derive_event_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_data = parse_event_struct(input)?;
    generate_event_impls(struct_data)
}
