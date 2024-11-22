use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::utils::{EmptyAttribute, EncodingFieldAttribute, StructItem};
use quote::quote;

pub type XmlStruct = StructItem<EncodingFieldAttribute, EmptyAttribute>;

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
