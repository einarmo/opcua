#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub enum StructureFieldType {
    Field(String),
    Array(String),
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct StructureField {
    pub name: String,
    pub typ: StructureFieldType,
}

#[cfg_attr(feature = "ser", derive(serde::Serialize))]
#[derive(Debug)]
pub struct StructuredType {
    pub name: String,
    pub fields: Vec<StructureField>,
    pub hidden_fields: Vec<String>,
    pub documentation: Option<String>,
    pub base_type: Option<String>,
    pub is_union: bool,
}

impl StructuredType {
    pub fn visible_fields(&self) -> impl Iterator<Item = &StructureField> {
        self.fields
            .iter()
            .filter(|f| !self.hidden_fields.contains(&f.name))
    }
}
