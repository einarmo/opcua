use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{
    parse_quote, punctuated::Punctuated, FieldsNamed, File, Generics, Ident, Item, ItemEnum,
    ItemImpl, ItemStruct, Lit, LitByte, Path, Token, Type, Visibility,
};

use crate::{error::CodeGenError, StructuredType};

use super::{loader::LoadedType, EnumType};
use quote::quote;

pub enum ItemDefinition {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

pub struct GeneratedItem {
    pub item: ItemDefinition,
    pub impls: Vec<ItemImpl>,
    pub module: String,
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
    import_map: HashMap<String, String>,
    input: Vec<LoadedType>,
    opcua_path: String,
}

impl CodeGenerator {
    pub fn new(
        json_serializable_types: HashSet<String>,
        external_import_map: HashMap<String, String>,
        input: Vec<LoadedType>,
        opcua_path: &str,
    ) -> Self {
        Self {
            json_serializable_types,
            import_map: external_import_map,
            input,
            opcua_path: opcua_path.to_owned(),
        }
    }

    pub fn generate_types(mut self) -> Result<Vec<GeneratedItem>, CodeGenError<'static>> {
        let mut generated = Vec::new();

        for item in &self.input {
            match item {
                LoadedType::Struct(s) => self.import_map.insert(
                    s.name.to_owned(),
                    format!("super::{}", s.name.to_case(Case::Snake)),
                ),
                LoadedType::Enum(s) => self
                    .import_map
                    .insert(s.name.to_owned(), "super::enums".to_owned()),
            };
        }
        let input = std::mem::take(&mut self.input);

        for item in input {
            match item {
                LoadedType::Struct(v) => generated.push(self.generate_struct(v)?),
                LoadedType::Enum(v) => generated.push(self.generate_enum(v)?),
            }
        }

        Ok(generated)
    }

    fn get_type_path(&self, name: &str) -> String {
        // Type is known, use the external path.
        if let Some(ext) = self.import_map.get(name) {
            return format!("{}::{}", ext, name);
        }
        // Assume the type is a builtin.
        name.to_string()
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
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

        let opcua_path: Path = syn::parse_str(&self.opcua_path)?;

        if item.values.iter().any(|f| f.name == "Invalid") {
            try_from_arms = quote! {
                #try_from_arms
                _ => Self::Invalid,
            };
        } else {
            try_from_arms = quote! {
                #try_from_arms
                _ => return Err(#opcua_path::types::StatusCode::BadUnexpectedError),
            };
        }

        let mut impls = Vec::new();
        let enum_ident = Ident::new(&item.name, Span::call_site());

        // TryFrom impl
        impls.push(parse_quote! {
            impl TryFrom<#ty> for #enum_ident {
                type Error = #opcua_path::types::StatusCode;

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
            impl #opcua_path::types::BinaryEncoder<#enum_ident> for #enum_ident {
                fn byte_len(&self) -> usize {
                    #size
                }

                fn encode<S: std::io::Write>(&self, stream: &mut S) -> #opcua_path::types::EncodingResult<usize> {
                    #opcua_path::types::#write_method(stream, *self as #ty)
                }

                fn decode<S: std::io::Read>(stream: &mut S, _: &#opcua_path::types::DecodingOptions) -> #opcua_path::types::EncodingResult<Self> {
                    let value = #opcua_path::types::#read_method(stream)?;
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
            module: "enums".to_string(),
        })
    }

    fn generate_struct(
        &self,
        item: StructuredType,
    ) -> Result<GeneratedItem, CodeGenError<'static>> {
        let mut attrs = Vec::new();
        let mut fields = Punctuated::new();

        if let Some(doc) = item.documentation {
            attrs.push(parse_quote! {
                #[doc = #doc]
            });
        }
        attrs.push(parse_quote! {
            #[derive(Debug, Clone, PartialEq)]
        });
        if self.json_serializable_types.contains(&item.name) {
            attrs.push(parse_quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
            });
            attrs.push(parse_quote! {
                #[serde(rename_all = "PascalCase")]
            });
        }

        let mut impls = Vec::new();
        let struct_ident = Ident::new(&item.name, Span::call_site());

        for field in &item.fields {
            let typ: Type = match &field.typ {
                crate::StructureFieldType::Field(f) => syn::parse_str(&self.get_type_path(&f))?,
                crate::StructureFieldType::Array(f) => {
                    let path: Path = syn::parse_str(&self.get_type_path(&f))?;
                    parse_quote! { Option<Vec<#path>> }
                }
            };
            let ident = Ident::new(&field.name, Span::call_site());
            fields.push(parse_quote! {
                pub #ident: #typ
            });
        }

        // Generate impls
        // Special case an empty struct
        let opcua_path: Path = syn::parse_str(&self.opcua_path)?;
        let mut len_impl;
        let mut encode_impl;
        let mut decode_impl = quote! {};
        let mut decode_build = quote! {};
        if item.fields.is_empty() {
            len_impl = quote! { 0usize };
            encode_impl = quote! { Ok(0) };
            decode_build = quote! { Ok(Self {}) };
        } else {
            len_impl = quote! {
                let mut size = 0usize;
            };
            encode_impl = quote! {
                let mut size = 0usize;
            };
            for field in &item.fields {
                let ident = Ident::new(&field.name, Span::call_site());

                match &field.typ {
                    crate::StructureFieldType::Field(f) => {
                        len_impl.extend(quote! {
                            size += self.#ident.byte_len();
                        });
                        encode_impl.extend(quote! {
                            size += self.#ident.encode(stream)?;
                        });
                        let ty: Type = syn::parse_str(&self.get_type_path(&f))?;
                        decode_impl.extend(quote! {
                            let #ident = <#ty as #opcua_path::types::BinaryEncoder<#ty>>::decode(stream, decoding_options)?;
                        });
                    }
                    crate::StructureFieldType::Array(_) => {
                        len_impl.extend(quote! {
                            size += #opcua_path::types::byte_len_array(&self.#ident);
                        });
                        encode_impl.extend(quote! {
                            size += #opcua_path::types::write_array(stream, &self.#ident)?;
                        });
                        decode_impl.extend(quote! {
                            let #ident = #opcua_path::types::read_array(stream, decoding_options)?;
                        });
                    }
                }

                decode_build.extend(quote! {
                    #ident,
                });
            }
            len_impl.extend(quote! {
                size
            });
            encode_impl.extend(quote! {
                Ok(size)
            });
            decode_build = quote! {
                Ok(Self {
                    #decode_build
                })
            };
        }

        impls.push(parse_quote! {
            impl #opcua_path::types::BinaryEncoder<#struct_ident> for #struct_ident {
                fn byte_len(&self) -> usize {
                    #len_impl
                }

                #[allow(unused_variables)]
                fn encode<S: std::io::Write>(&self, stream: &mut S) -> #opcua_path::types::EncodingResult<usize> {
                    #encode_impl
                }

                #[allow(unused_variables)]
                fn decode<S: std::io::Read>(stream: &mut S, decoding_options: &#opcua_path::types::DecodingOptions) -> #opcua_path::types::EncodingResult<Self> {
                    #decode_impl
                    #decode_build
                }
            }
        });

        let res = ItemStruct {
            attrs,
            vis: Visibility::Public(Token![pub](Span::call_site())),
            struct_token: Token![struct](Span::call_site()),
            ident: struct_ident,
            generics: Generics::default(),
            fields: syn::Fields::Named(FieldsNamed {
                brace_token: syn::token::Brace(Span::call_site()),
                named: fields,
            }),
            semi_token: None,
        };

        Ok(GeneratedItem {
            item: ItemDefinition::Struct(res),
            impls,
            module: item.name.to_case(Case::Snake),
        })
    }
}
