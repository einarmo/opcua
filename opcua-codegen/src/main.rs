use opcua_codegen::{run_codegen, CodeGenConfig, CodeGenError};
use opcua_xml::{load_nodeset2_file, schema::xml_schema::load_xsd_schema};

fn main() -> Result<(), CodeGenError> {
    // run_cli()?;
    let types_xml = std::fs::read_to_string("tools/schema/schemas/1.0.4/Opc.Ua.Types.xsd").unwrap();
    let schema = load_xsd_schema(&types_xml)?;

    for item in schema.items {
        println!("{:?}", item);
    }

    let node_set =
        std::fs::read_to_string("tools/schema/schemas/1.0.4/Opc.Ua.NodeSet2.xml").unwrap();
    let node_set = load_nodeset2_file(&node_set)?;

    let nodes = node_set.node_set.unwrap();
    println!("{}", nodes.nodes.len());

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
