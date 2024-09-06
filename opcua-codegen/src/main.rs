use opcua_codegen::{run_codegen, CodeGenConfig, CodeGenError};

fn main() -> Result<(), CodeGenError> {
    run_cli()
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
