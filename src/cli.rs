use clap::{crate_description, crate_name, crate_version, App, Arg, ArgMatches};

pub fn parse() -> ArgMatches {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .args(&[
            Arg::new("input")
                .help("files/directory to process")
                .index(1)
                .required(true),
            Arg::new("replace")
                .short('r')
                .long("replace")
                .help("sort and replace files")
                .required(false)
                .takes_value(false),
            Arg::new("out")
                .short('o')
                .long("out")
                .help("output directory ( ignores suffix )")
                .takes_value(true)
                .required(false),
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .help("suffix for output files")
                .default_value("sorted")
                .required(false),
        ]);
    app.get_matches()
}
