use std::process;
extern crate clap;
#[macro_use] extern crate log;
use clap::{App, Arg};
use std::{fs, io};
use std::io::Read;

const VERSION: &str = concat!("v", clap::crate_version!());
const LONG_ABOUT: &str = r#"
Creates an ASCII encoded table out of a JSON specification.
As input we are able to read from stdin or from a file.

Originally it was intended to be piped to some kind of Jira
query, which would return a JSON table, possibly after being
filtered with jq.
"#;

fn main() {
    env_logger::init();
    let matches = App::new(clap::crate_name!())
                                   .author(clap::crate_authors!())
                                   .version(VERSION)
                                   .about(clap::crate_description!())
                                   .long_about(LONG_ABOUT)
                                   .arg(Arg::with_name("json")
                                       .short("j")
                                       .long("json")
                                       .help("Input file if not using STDIN")
                                       .takes_value(true))
                                   .get_matches();
    let input_file = matches.value_of("json");
    let contents = match input_file {
        Some(filename) => {
            fs::read_to_string(filename).expect("Could not read input file")
        },
        None => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buffer).expect("Could not read STDIN");
            buffer
        }
    };
    if let Err(e) = json_to_table::print_table(&contents) {
        eprintln!("{}", e);
        process::exit(2);
    }
}
