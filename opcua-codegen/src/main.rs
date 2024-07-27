use std::{collections::HashMap, io::Write};

use opcua_codegen::{nodeset::NodeSetCodeGenerator, run_codegen, CodeGenConfig, CodeGenError};
use opcua_xml::{
    load_nodeset2_file,
    schema::xml_schema::{load_xsd_schema, XsdFileItem, XsdFileType},
};

fn main() -> Result<(), CodeGenError> {
    // run_cli()?;

    let node_set =
        std::fs::read_to_string("tools/schema/schemas/1.0.4/Opc.Ua.NodeSet2.xml").unwrap();
    let node_set = load_nodeset2_file(&node_set)?;

    let nodes = node_set.node_set.unwrap();

    let xsd_file = std::fs::read_to_string("tools/schema/schemas/1.0.4/Opc.Ua.Types.xsd").unwrap();
    println!("{}", &xsd_file[7278..7427]);
    let xsd_file = load_xsd_schema(&xsd_file)?;

    let mut types = HashMap::new();
    for it in xsd_file.items {
        match it {
            XsdFileItem::SimpleType(i) => {
                if let Some(name) = i.name.clone() {
                    types.insert(name, XsdFileType::Simple(i));
                }
            }
            XsdFileItem::ComplexType(i) => {
                if let Some(name) = i.name.clone() {
                    types.insert(name, XsdFileType::Complex(i));
                }
            }
            XsdFileItem::Element(_) => (),
        }
    }

    let mut generator = NodeSetCodeGenerator::new("opcua", "", nodes.aliases, types)?;

    let mut fns = Vec::new();
    for node in nodes.nodes.into_iter() {
        let fun = generator.generate_item(node)?;
        fns.push(syn::Item::Fn(fun.func));
    }

    let out_file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: fns,
    };

    let mut file = std::fs::File::options()
        .create(true)
        .write(true)
        .open("samples/gen-test/src/gen.rs")?;
    file.write_all(prettyplease::unparse(&out_file).as_bytes())
        .unwrap();

    Ok(())
}

fn run_cli() -> Result<(), CodeGenError> {
    let args = std::env::args();

    if args.len() != 2 {
        println!(
            r#"Usage:
opcua-codegen [config].yml
"#
        );
        return Ok(());
    }

    let config_path = args.skip(1).next().unwrap();

    let config_text =
        std::fs::read_to_string(config_path).expect("Failed to read config from file");
    let config: CodeGenConfig =
        serde_yaml::from_str(&config_text).expect("Failed to parse config file");

    run_codegen(&config)?;

    Ok(())
}
