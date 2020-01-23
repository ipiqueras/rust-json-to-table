use std::process;
extern crate clap;
#[macro_use] extern crate log;
use clap::{App, Arg};
use std::fs;

const VERSION: &str = concat!("v", clap::crate_version!());
const LONG_ABOUT: &str = r#"
Creates a LaTeX encoded table out of a JSON specification. 
As input we are able to read from stdin or from a file.

We encode special characters before writing.
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
    if let Some(filename) = input_file {
        let contents = fs::read_to_string(filename)
            .expect("Could not read input file");
        if let Err(e) = jira::run(&contents) {
            eprintln!("{}", e);
            process::exit(2);
        }
    } // else read from std input
}
