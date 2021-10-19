mod cli;
mod gen;
mod output;
use gen::Gen;
use ignore::{types::TypesBuilder, WalkBuilder};
use json::{codegen::Generator, JsonValue};
use std::{fs::read_to_string, io::Write};

fn main() {
    let matches = cli::parse();

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
            println!("Got here : {}", dir.path().display());
            let file = read_to_string(dir.path()).unwrap();
            println!("Found data : {}", file);
            let data: JsonValue = json::parse(file.as_str()).expect("failed to parse");
            let mut gen = Gen::new();
            gen.write_json(&data).unwrap();
            let res = gen.consume();

            let suffix = matches.value_of("suffix").unwrap();
            let mut f = output::with_suffix(suffix, dir.path());
            if matches.is_present("out") {
                f = output::with_dir(matches.value_of("out").unwrap_or("out"), dir.path())
            }
            if matches.is_present("replace") {
                f = output::with_replace(dir.path())
            }
            f.write_all(res.as_bytes()).unwrap();
            println!("Written {}", dir.path().display());
        }
    });
}
