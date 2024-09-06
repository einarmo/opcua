use collector::TypeCollector;
use gen::EventGenerator;
use opcua_xml::schema::ua_node_set::UANodeSet;
use syn::Item;

use crate::base_native_type_mappings;

mod collector;
mod gen;

pub fn test(nodeset: &UANodeSet) {
    let coll = TypeCollector::new(nodeset.nodes.iter(), nodeset.aliases.as_ref());
    let collected = coll.collect_types().unwrap();

    let gen = EventGenerator::new(collected, &[], base_native_type_mappings());
    let items = gen.render().unwrap();
    let items: Vec<_> = items.into_iter().map(|v| Item::Struct(v.def)).collect();
    let file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items,
    };

    println!("{}", prettyplease::unparse(&file));
}
