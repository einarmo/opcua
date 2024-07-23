use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
};

use opcua_codegen::{
    create_module_file, default_bsd_type_loader, default_code_generator, CodeGenItemConfig,
};
use opcua_xml::load_bsd_file;

fn main() {
    let path = "../tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd";
    let mut file = File::open(path).unwrap();
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);

    file.read_to_string(&mut buf).unwrap();

    let data = load_bsd_file(&buf).unwrap();

    let config = CodeGenItemConfig {
        enums_single_file: true,
        structs_single_file: false,
        opcua_crate_path: "opcua".to_owned(),
    };

    let type_gen = default_bsd_type_loader(data).unwrap();
    let res = type_gen.from_bsd().unwrap();
    //let pretty = serde_json::to_string_pretty(&res).unwrap();
    //println!("{pretty}");

    let generator = default_code_generator(res, config);
    let generated = generator.generate_types().unwrap();

    std::fs::remove_dir_all("../samples/gen-test/src/generated").unwrap();
    std::fs::create_dir_all("../samples/gen-test/src/generated").unwrap();
    let mut modules = HashSet::new();
    for gen in generated {
        let mut file = File::options()
            .append(true)
            .create(true)
            .open(format!(
                "../samples/gen-test/src/generated/{}.rs",
                gen.module
            ))
            .unwrap();
        modules.insert(gen.module.clone());
        file.write_all(&prettyplease::unparse(&gen.to_file()).as_bytes())
            .unwrap();
    }

    let mut file = File::options()
        .append(true)
        .create(true)
        .open("../samples/gen-test/src/generated/mod.rs")
        .unwrap();
    let module_file = create_module_file(modules);
    file.write_all(&prettyplease::unparse(&module_file).as_bytes())
        .unwrap();
}
