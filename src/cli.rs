use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("wget-01")
        .version("1.0")
        .author("cheimbaye & annndiaye")
        .about("Recreate certain features of WGET in Rust")
        .arg(
            Arg::new("url")
                .help("The URL of the file to download")
                .value_parser(clap::value_parser!(String)) // Utilisation de value_parser pour accepter les valeurs
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("output")
                .short('O')
                .long("output")
                .help("Name under which the file is saved")
                .value_name("NAME"),
        )
        .arg(
            Arg::new("path")
                .short('P')
                .long("path")
                .help("Directory where the file is saved")
                .value_name("DIRECTORY"),
        )
}
