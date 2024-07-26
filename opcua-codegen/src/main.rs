use std::io::Write;

use opcua_codegen::{nodeset::NodeSetCodeGenerator, run_codegen, CodeGenConfig, CodeGenError};
use opcua_xml::{load_nodeset2_file, schema::ua_node_set::UANode};
use quote::ToTokens;

fn main() -> Result<(), CodeGenError> {
    // run_cli()?;

    let node_set =
        std::fs::read_to_string("tools/schema/schemas/1.0.4/Opc.Ua.NodeSet2.xml").unwrap();
    let node_set = load_nodeset2_file(&node_set)?;

    let nodes = node_set.node_set.unwrap();

    let mut generator = NodeSetCodeGenerator::new("opcua", "", nodes.aliases)?;

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
