use collector::TypeCollector;
use gen::{EventGenerator, EventItem};
use opcua_xml::schema::ua_node_set::UANodeSet;
use syn::Item;

use crate::{base_native_type_mappings, CodeGenError, GeneratedOutput};

mod collector;
mod gen;

pub fn generate_events(nodeset: &UANodeSet) -> Result<Vec<EventItem>, CodeGenError> {
    let coll = TypeCollector::new(nodeset.nodes.iter(), nodeset.aliases.as_ref());
    let collected = coll.collect_types()?;

    let gen = EventGenerator::new(collected, &[], base_native_type_mappings());
    let items = gen.render()?;
    Ok(items)
}

impl GeneratedOutput for EventItem {
    fn module(&self) -> &str {
        "generated"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn to_file(self) -> syn::File {
        syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: vec![Item::Struct(self.def)],
        }
    }
}
