extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use cbindgen::Config;


fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.h", package_name))
        .display()
        .to_string();

    let config = Config {
        language: cbindgen::Language::C,
        parse: cbindgen::ParseConfig {
            parse_deps: true,
            include: Option::from(vec!["libgbrainy".to_string()]),
            exclude: vec![],
            expand: Default::default(),
            clean: false,
            extra_bindings: vec![],
        },
        // export: cbindgen::ExportConfig {
        //     // include: vec![],
        //     // exclude: vec![],
        //     // rename: Default::default(),
        //     // pre_body: Default::default(),
        //     // body: Default::default(),
        //     prefix: Some("rustybrain".to_string()),
        //     // item_types: vec![cbindgen::ItemType::Constants,
        //     //                  cbindgen::ItemType::Structs,
        //     //                  cbindgen::ItemType::Functions,
        //     //                  cbindgen::ItemType::OpaqueItems,
        //     //                  cbindgen::ItemType::Enums,
        //     //                  cbindgen::ItemType::Globals,
        //     //                  cbindgen::ItemType::Typedefs
        //     // ],
        //     // renaming_overrides_prefixing: false,
        //     mangle: Default::default(),
        //     ..Default::default()
        // },
        ..Default::default()
    };

    let res = cbindgen::generate_with_config(&crate_dir, config);
    if res.is_ok() {
        res.unwrap().write_to_file(&output_file);
    }else{
        match res.err(){
            Some(s) => println!("{}", s.to_string()),
            None => todo!(),
        }
    
    }
    
       
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}