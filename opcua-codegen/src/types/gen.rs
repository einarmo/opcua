use std::collections::HashSet;

use proc_macro2::Span;
use syn::{
    parse_quote, punctuated::Punctuated, File, Generics, Ident, Item, ItemEnum, ItemImpl,
    ItemStruct, Lit, LitByte, LitInt, Token, Type, Visibility,
};

use crate::error::CodeGenError;

use super::{loader::LoadedType, EnumType};
use quote::quote;

pub enum ItemDefinition {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

pub struct GeneratedItem {
    pub item: ItemDefinition,
    pub impls: Vec<ItemImpl>,
}

impl GeneratedItem {
    pub fn to_file(self) -> File {
        let mut items = Vec::new();
        match self.item {
            ItemDefinition::Struct(v) => items.push(Item::Struct(v)),
            ItemDefinition::Enum(v) => items.push(Item::Enum(v)),
        }
        for imp in self.impls {
            items.push(Item::Impl(imp));
        }

        File {
            shebang: None,
            attrs: Vec::new(),
            items,
        }
    }
}

pub struct CodeGenerator {
    json_serializable_types: HashSet<String>,
}

impl CodeGenerator {
    pub fn new(json_serializable_types: HashSet<String>) -> Self {
        Self {
            json_serializable_types,
        }
    }

    pub fn generate_types(
        &self,
        input: Vec<LoadedType>,
    ) -> Result<Vec<GeneratedItem>, CodeGenError<'static>> {
        let mut generated = Vec::new();

        for item in input {
            match item {
                LoadedType::Struct(_) => continue,
                LoadedType::Enum(v) => generated.push(self.generate_enum(v)?),
            }
        }

        Ok(generated)
    }

    fn generate_enum(&self, item: EnumType) -> Result<GeneratedItem, CodeGenError<'static>> {
        let mut attrs = Vec::new();
        let mut variants = Punctuated::new();

        if let Some(doc) = item.documentation {
            attrs.push(parse_quote! {
                #[doc = #doc]
            });
        }
        attrs.push(parse_quote! {
            #[derive(Debug, Copy, Clone, PartialEq)]
        });
        if self.json_serializable_types.contains(&item.name) {
            attrs.push(parse_quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
            });
            attrs.push(parse_quote! {
                #[serde(rename_all = "PascalCase")]
            });
        }
        let ty: Type = syn::parse_str(&item.typ.to_string())?;
        attrs.push(parse_quote! {
            #[repr(#ty)]
        });

        let mut try_from_arms = quote! {};

        for field in &item.values {
            let name = Ident::new(&field.name, Span::call_site());
            let value = field.value;
            let value_token = match item.typ {
                super::enum_type::EnumReprType::u8 => {
                    let value: u8 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to u8, {} is out of range",
                            value
                        ))
                    })?;
                    Lit::Byte(LitByte::new(value, Span::call_site()))
                }
                super::enum_type::EnumReprType::i16 => {
                    let value: i16 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to i16, {} is out of range",
                            value
                        ))
                    })?;
                    parse_quote! { #value }
                }
                super::enum_type::EnumReprType::i32 => {
                    let value: i32 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to i32, {} is out of range",
                            value
                        ))
                    })?;
                    parse_quote! { #value }
                }
                super::enum_type::EnumReprType::i64 => {
                    parse_quote! { #value }
                }
            };

            try_from_arms = quote! {
                #try_from_arms
                #value_token => Self::#name,
            };

            variants.push(parse_quote! {
                #name = #value_token
            })
        }

        if item.values.iter().any(|f| f.name == "Invalid") {
            try_from_arms = quote! {
                #try_from_arms
                _ => Self::Invalid,
            };
        } else {
            try_from_arms = quote! {
                #try_from_arms
                _ => return Err(opcua::types::StatusCode::BadUnexpectedError),
            };
        }

        let mut impls = Vec::new();
        let enum_ident = Ident::new(&item.name, Span::call_site());

        // TryFrom impl
        impls.push(parse_quote! {
            impl TryFrom<#ty> for #enum_ident {
                type Error = opcua::types::StatusCode;

                fn try_from(value: #ty) -> Result<Self, <Self as TryFrom<#ty>>::Error> {
                    Ok(match value {
                        #try_from_arms
                    })
                }
            }
        });

        // BinaryEncoder impl
        let size: usize = item.size.try_into().map_err(|_| {
            CodeGenError::Other(format!("Value {} does not fit in a usize", item.size))
        })?;
        let write_method = Ident::new(&format!("write_{}", item.typ), Span::call_site());
        let read_method = Ident::new(&format!("read_{}", item.typ), Span::call_site());

        impls.push(parse_quote! {
            impl opcua::types::BinaryEncoder<#enum_ident> for #enum_ident {
                fn byte_len(&self) -> usize {
                    #size
                }

                fn encode<S: std::io::Write>(&self, stream: &mut S) -> opcua::types::EncodingResult<usize> {
                    opcua::types::#write_method(stream, *self as #ty)
                }

                fn decode<S: std::io::Read>(stream: &mut S, _: &opcua::types::DecodingOptions) -> opcua::types::EncodingResult<Self> {
                    let value = opcua::types::#read_method(stream)?;
                    Ok(Self::try_from(value)?)
                }
            }
        });

        let res = ItemEnum {
            attrs,
            vis: Visibility::Public(Token![pub](Span::call_site())),
            enum_token: Token![enum](Span::call_site()),
            ident: enum_ident,
            generics: Generics::default(),
            brace_token: syn::token::Brace(Span::call_site()),
            variants,
        };

        Ok(GeneratedItem {
            item: ItemDefinition::Enum(res),
            impls,
        })
    }
}
