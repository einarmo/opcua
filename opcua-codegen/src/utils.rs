use proc_macro2::Span;
use syn::{parse_quote, File, Ident};

pub fn create_module_file(modules: Vec<String>) -> File {
    let mut items = Vec::new();
    for md in modules {
        let ident = Ident::new(&md, Span::call_site());
        items.push(parse_quote! {
            pub mod #ident;
        });
        items.push(parse_quote! {
            pub use #ident::*;
        });
    }

    File {
        shebang: None,
        attrs: Vec::new(),
        items,
    }
}

pub trait GeneratedOutput {
    fn to_file(self) -> File;

    fn module(&self) -> &str;

    fn name(&self) -> &str;
}
