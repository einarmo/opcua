use syn::{parse::Parse, DeriveInput, Field, Ident, LitStr, Token, Type};

#[derive(Debug, Default)]
pub struct EmptyAttribute;

impl Parse for EmptyAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Unexpected attribute"));
        }
        Ok(EmptyAttribute)
    }
}

impl ItemAttr for EmptyAttribute {
    fn combine(&mut self, _other: Self) {}
}

pub trait ItemAttr {
    fn combine(&mut self, other: Self);
}

pub struct StructField<T> {
    pub ident: Ident,
    pub typ: Type,
    pub attr: T,
}

pub struct StructItem<TFieldAttr, TAttr> {
    pub ident: Ident,
    pub fields: Vec<StructField<TFieldAttr>>,
    pub attribute: TAttr,
}

#[derive(Debug, Default)]
pub(super) struct EncodingFieldAttribute {
    pub rename: Option<String>,
    pub ignore: bool,
    pub required: bool,
}

impl Parse for EncodingFieldAttribute {
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
                "ignore" => {
                    slf.ignore = true;
                }
                "required" => {
                    slf.required = true;
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

impl ItemAttr for EncodingFieldAttribute {
    fn combine(&mut self, other: Self) {
        self.rename = other.rename;
        self.ignore |= other.ignore;
        self.required |= other.required;
    }
}

impl<TFieldAttr: Parse + ItemAttr + Default, TAttr: Parse + ItemAttr + Default>
    StructItem<TFieldAttr, TAttr>
{
    pub fn from_input(input: DeriveInput) -> syn::Result<Self> {
        let strct = match input.data {
            syn::Data::Struct(s) => s,
            _ => {
                return Err(syn::Error::new_spanned(
                    input.ident,
                    "Derive macro input must be a struct",
                ));
            }
        };

        let fields = strct
            .fields
            .into_iter()
            .map(StructField::from_field)
            .collect::<Result<Vec<_>, _>>()?;

        let mut final_attr = TAttr::default();
        for attr in input.attrs {
            if attr.path().segments.len() == 1
                && attr
                    .path()
                    .segments
                    .first()
                    .is_some_and(|s| s.ident == "opcua")
            {
                let data: TAttr = attr.parse_args()?;
                final_attr.combine(data);
            }
        }

        Ok(Self {
            ident: input.ident,
            fields,
            attribute: final_attr,
        })
    }
}

impl<T: Parse + ItemAttr + Default> StructField<T> {
    pub fn from_field(field: Field) -> syn::Result<Self> {
        let Some(ident) = field.ident else {
            return Err(syn::Error::new_spanned(
                field,
                "Derive macro input must have named fields",
            ));
        };
        let mut final_attr = T::default();
        for attr in field.attrs {
            if attr.path().segments.len() == 1
                && attr
                    .path()
                    .segments
                    .first()
                    .is_some_and(|s| s.ident == "opcua")
            {
                let data: T = attr.parse_args()?;
                final_attr.combine(data);
            }
        }
        Ok(StructField {
            ident,
            typ: field.ty,
            attr: final_attr,
        })
    }
}
