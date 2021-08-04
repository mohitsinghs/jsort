mod gen;
use gen::Gen;
use json::{codegen::Generator, JsonValue};
use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use ignore::{types::TypesBuilder, WalkBuilder};

fn main() {
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("input")
                .about("files/directory to process")
                .index(1)
                .required(true),
        );
    let matches = cli.get_matches();

    // build types
    let files: &str = matches.value_of("input").expect("Input can't be empty");
    let mut tb = TypesBuilder::new();
    tb.add_def("json:*.json").expect("Failed to add types");
    let types = tb
        .select("json")
        .build()
        .expect("Failed to build filetypes");

    let walk = WalkBuilder::new(files).types(types).build();
    walk.filter_map(|d| d.ok()).for_each(|dir| {
        if dir.path().is_file() {
            let file = read_to_string(dir.path()).unwrap();
            let data: JsonValue = json::parse(file.as_str()).unwrap();
            let mut gen = Gen::new();
            gen.write_json(&data).unwrap();
            let res = gen.consume();
            let mut f = OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(dir.path())
                .unwrap();
            f.write_all(res.as_bytes()).unwrap();
            println!("Written {}", dir.path().display());
        }
    });
}
