use proc_macro2::{Span, TokenStream};
use syn::{parse_quote, File, Ident};

use crate::CodeGenError;

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

pub trait RenderExpr {
    fn render(&self) -> Result<TokenStream, CodeGenError>;
}

impl<T> RenderExpr for Option<&T>
where
    T: RenderExpr,
{
    fn render(&self) -> Result<TokenStream, CodeGenError> {
        Ok(match self {
            Some(t) => {
                let rendered = t.render()?;
                parse_quote! {
                    Some(#rendered)
                }
            }
            None => parse_quote! { None },
        })
    }
}
