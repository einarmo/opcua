use super::parse::{EventStruct, Identifier};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_event_impls(event: EventStruct) -> syn::Result<TokenStream> {
    let ident = event.ident;
    let mut get_arms = quote! {};
    let mut init_items = quote! {};
    let mut extra_init_args = quote! {};
    for field in event.fields {
        let name = field
            .attr
            .rename
            .unwrap_or_else(|| field.ident.to_string().to_case(Case::Pascal));
        let ident = field.ident;
        if !field.attr.ignore {
            get_arms.extend(quote! {
                #name => self.#ident.get_value(attribute_id, index_range, browse_path.get(1..).unwrap_or(&[])),
            });
        }
        let ty = field.typ;
        if field.attr.required {
            extra_init_args.extend(quote! {
                #ident: #ty,
            });
            init_items.extend(quote! {
                #ident,
            });
        } else {
            init_items.extend(quote! {
                #ident: Default::default(),
            });
        }
    }

    let Some(idf) = event.attribute.identifier else {
        return Err(syn::Error::new_spanned(
            ident,
            "Event must have an attribute `#[opcua(identifier = \"...\")]",
        ));
    };
    let type_id_body = match idf {
        Identifier::Number(i) => quote! {
            type_definition_id == &(self.own_namespace_index, #i)
        },
        Identifier::String(s) => quote! {
            type_definition_id == &(self.own_namespace_index, #s)
        },
        Identifier::Guid(v) => {
            let bytes = v.as_bytes();
            quote! {
                let idf: &[u8; 16] = &[#(#bytes),*];
                type_definition_id == &(self.own_namespace_index, idf)
            }
        }
        Identifier::ByteString(v) => {
            quote! {
                let idf: &[u8] = &[#(#v),*];
                type_definition_id == &(self.own_namespace_index, idf)
            }
        }
    };

    Ok(quote! {
        impl opcua::server::Event for #ident {
            fn get_field(
                &self,
                type_definition_id: &NodeId,
                attribute_id: opcua::types::AttributeId,
                index_range: opcua::types::NumericRange,
                browse_path: &[opcua::types::QualifiedName],
            ) -> opcua::types::Variant {
                use opcua::server::EventField;

                if type_definition_id != &opcua::types::ObjectTypeId::BaseEventType && !{
                    #type_id_body
                } {
                    return self.base.get_field(
                        type_definition_id, attribute_id, index_range, browse_path
                    );
                }

                if browse_path.is_empty() {
                    return opcua::types::Variant::Empty;
                }
                let field = &browse_path[0];
                let rest = browse_path.get(1..).unwrap_or(&[]);
                match field.name.as_ref() {
                    #get_arms
                    _ => self.base.get_field(type_definition_id, attribute_id, index_range, browse_path)
                }
            }

            fn time(&self) -> &opcua::types::DateTime {
                self.base.time()
            }
        }
    })
}
