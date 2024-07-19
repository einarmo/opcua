use std::{fs::File, io::Read};

use opcua_codegen::{default_code_generator, default_type_loader};

fn main() {
    let path = "../tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd";
    let mut file = File::open(path).unwrap();
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);

    file.read_to_string(&mut buf).unwrap();

    let type_gen = default_type_loader(&buf).unwrap();
    let res = type_gen.from_bsd().unwrap();
    //let pretty = serde_json::to_string_pretty(&res).unwrap();
    //println!("{pretty}");

    let generator = default_code_generator();
    let generated = generator.generate_types(res).unwrap();

    for gen in generated {
        println!("{}", prettyplease::unparse(&gen.to_file()))
    }
}
