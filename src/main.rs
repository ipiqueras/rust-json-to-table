extern crate clap;
#[macro_use] extern crate log;

const VERSION: &str = concat!("v", clap::crate_version!());
const LONG_ABOUT: &str = r#"
Creates a LaTeX encoded table out of a JSON specification. 
As input we are able to read from stdin or from a file.

We encode special characters before writing.
"#;

fn main() {
    env_logger::init();
    clap::App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(VERSION)
        .about(clap::crate_description!())
        .long_about(LONG_ABOUT)
        .get_matches();
    println!("Hello, world!");
}
