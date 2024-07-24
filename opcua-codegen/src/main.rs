use opcua_codegen::{run_codegen, CodeGenConfig, CodeGenTarget, TypeCodeGenTarget};

fn main() {
    let config = CodeGenConfig {
        extra_header: r#"// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang"#
            .to_owned(),
        opcua_crate_path: "crate".to_owned(),
        targets: vec![CodeGenTarget::Types(TypeCodeGenTarget {
            file_path: "tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd".to_owned(),
            output_dir: "lib/src/types/service_types".to_owned(),
            enums_single_file: true,
            default_excluded: ["AnonymousIdentityToken"]
                .into_iter()
                .map(|t| t.to_owned())
                .collect(),
            ..Default::default()
        })],
    };

    run_codegen(&config).unwrap();
}
