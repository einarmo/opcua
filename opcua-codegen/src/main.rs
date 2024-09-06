use opcua_codegen::{nodeset::test, run_codegen, CodeGenConfig, CodeGenError};
use opcua_xml::load_nodeset2_file;

fn main() -> Result<(), CodeGenError> {
    // run_cli()
    let node_set =
        std::fs::read_to_string("tools/schema/schemas/1.05/Opc.Ua.NodeSet2.xml").unwrap();
    let node_set = load_nodeset2_file(&node_set)?;
    test(&node_set.node_set.unwrap());

    Ok(())
}

fn run_cli() -> Result<(), CodeGenError> {
    let mut args = std::env::args();

    if args.len() != 2 {
        println!(
            r#"Usage:
opcua-codegen [config].yml
"#
        );
        return Ok(());
    }

    let config_path = args.nth(1).unwrap();

    let config_text =
        std::fs::read_to_string(config_path).expect("Failed to read config from file");
    let config: CodeGenConfig =
        serde_yaml::from_str(&config_text).expect("Failed to parse config file");

    run_codegen(&config)?;

    Ok(())
}
