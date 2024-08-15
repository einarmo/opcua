use opcua_codegen::{run_codegen, CodeGenConfig, CodeGenError};

fn main() -> Result<(), CodeGenError> {
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

    run_codegen(&config).unwrap();

    Ok(())
}
