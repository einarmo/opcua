use syn::{parse::Parse, DeriveInput, Field, Ident, Type};

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
                    .is_some_and(|s| s.ident.to_string() == "opcua")
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
                    .is_some_and(|s| s.ident.to_string() == "opcua")
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
