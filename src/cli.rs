use clap::{crate_description, crate_name, crate_version, Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    let cmd = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .args(&[
            Arg::new("input")
                .help("files/directory to process")
                .required(true),
            Arg::new("replace")
                .short('r')
                .long("replace")
                .conflicts_with_all(&["out", "suffix"])
                .help("sort and replace files"),
            Arg::new("out")
                .short('o')
                .long("out")
                .conflicts_with_all(&["replace", "suffix"])
                .help("output directory")
                .takes_value(true)
                .min_values(0)
                .require_equals(true)
                .default_missing_value("out"),
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .help("suffix for output files")
                .takes_value(true)
                .min_values(0)
                .require_equals(true)
                .default_missing_value("sorted"),
        ]);
    cmd.get_matches()
}
