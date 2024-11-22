use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::{parse::Parse, DeriveInput, Ident, LitStr, Token};

use crate::utils::{EmptyAttribute, ItemAttr, StructItem};
use quote::quote;

#[derive(Debug, Default)]
pub(super) struct XmlFieldAttribute {
    pub rename: Option<String>,
}

impl Parse for XmlFieldAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut slf = Self::default();

        loop {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "rename" => {
                    input.parse::<Token![=]>()?;
                    let val: LitStr = input.parse()?;
                    slf.rename = Some(val.value());
                }
                "ignore" | "required" => (),
                _ => return Err(syn::Error::new_spanned(ident, "Unknown attribute value")),
            }
            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(slf)
    }
}

impl ItemAttr for XmlFieldAttribute {
    fn combine(&mut self, other: Self) {
        self.rename = other.rename;
    }
}

pub type XmlStruct = StructItem<XmlFieldAttribute, EmptyAttribute>;

pub fn parse_xml_struct(input: DeriveInput) -> syn::Result<XmlStruct> {
    XmlStruct::from_input(input)
}

pub fn generate_xml_impl(strct: XmlStruct) -> syn::Result<TokenStream> {
    let ident = strct.ident;
    let mut body = quote! {};
    let mut build = quote! {};
    for field in strct.fields {
        let name = field
            .attr
            .rename
            .unwrap_or_else(|| field.ident.to_string().to_case(Case::Pascal));
        let ident = field.ident;
        body.extend(quote! {
            let #ident = opcua::types::xml::XmlField::get_xml_field(element, #name, ctx)?;
        });
        build.extend(quote! {
            #ident,
        });
    }
    Ok(quote! {
        impl opcua::types::xml::FromXml for #ident {
            fn from_xml<'a>(
                element: &opcua::types::xml::XmlElement,
                ctx: &opcua::types::xml::XmlContext<'a>
            ) -> Result<Self, opcua::types::xml::FromXmlError> {
                #body
                Ok(Self {
                    #build
                })
            }
        }
    })
}
