use std::str::FromStr;

use base64::Engine;
use syn::{parse::Parse, DeriveInput, Ident, LitStr, Token};
use uuid::Uuid;

use crate::utils::{ItemAttr, StructItem};

#[derive(Default, Debug)]
pub(super) struct EventFieldAttribute {
    pub ignore: bool,
    pub required: bool,
    pub rename: Option<String>,
}

impl Parse for EventFieldAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut slf = Self::default();
        loop {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "ignore" => slf.ignore = true,
                "required" => slf.required = true,
                "rename" => {
                    input.parse::<Token![=]>()?;
                    let val: LitStr = input.parse()?;
                    slf.rename = Some(val.value());
                }
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

impl ItemAttr for EventFieldAttribute {
    fn combine(&mut self, other: Self) {
        self.ignore |= other.ignore;
        self.required |= other.required;
        if other.rename.is_some() {
            self.rename = other.rename.clone();
        }
    }
}

pub(super) enum Identifier {
    Number(u32),
    String(String),
    Guid(Uuid),
    ByteString(Vec<u8>),
}

impl FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err("Identifier not on form i=..., s=..., g=..., or o=...".to_owned());
        }

        let start = &s[0..2];
        let rest = &s[2..];
        Ok(match start {
            "i=" => Identifier::Number(
                rest.parse()
                    .map_err(|e| format!("Invalid identifier: {e}"))?,
            ),
            "s=" => Identifier::String(rest.to_owned()),
            "g=" => Identifier::Guid(
                uuid::Uuid::parse_str(rest).map_err(|e| format!("Invalid identfier: {e}"))?,
            ),
            "o=" => Identifier::ByteString(
                base64::engine::general_purpose::STANDARD
                    .decode(rest)
                    .map_err(|e| format!("Invalid identfier: {e}"))?,
            ),
            _ => return Err("Identifier not on form i=..., s=..., g=..., or o=...".to_owned()),
        })
    }
}

#[derive(Default)]
pub(super) struct EventAttribute {
    pub identifier: Option<Identifier>,
}

impl Parse for EventAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let idf: Option<Identifier>;

        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "identifier" => {
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                idf = Some(
                    Identifier::from_str(&lit.value())
                        .map_err(|e| syn::Error::new_spanned(lit, e))?,
                );
            }
            _ => return Err(syn::Error::new_spanned(ident, "Unknown attribute value")),
        }

        Ok(Self { identifier: idf })
    }
}

impl ItemAttr for EventAttribute {
    fn combine(&mut self, other: Self) {
        self.identifier = other.identifier;
    }
}

pub type EventStruct = StructItem<EventFieldAttribute, EventAttribute>;

pub fn parse_event_struct(input: DeriveInput) -> syn::Result<EventStruct> {
    let mut parsed = EventStruct::from_input(input)?;

    let mut filtered_fields = Vec::with_capacity(parsed.fields.len() - 2);

    let mut has_base = false;
    let mut has_own_idx = false;
    for field in parsed.fields.drain(..) {
        let name = field.ident.to_string();
        if name == "base" {
            has_base = true;
            continue;
        }
        if name == "own_namespace_index" {
            has_own_idx = true;
            continue;
        }
        filtered_fields.push(field);
    }

    parsed.fields = filtered_fields;

    if !has_base {
        return Err(syn::Error::new_spanned(
            parsed.ident,
            "Event must contain a field `base` that implements `Event`",
        ));
    }
    if !has_own_idx {
        return Err(syn::Error::new_spanned(
            parsed.ident,
            "Event must contain a field `own_namespace_index` of type `u16`",
        ));
    }

    return Ok(parsed);
}
