extern crate atty;
#[macro_use]
extern crate clap;
extern crate gitconfig2json;

use atty::Stream;
use clap::{Arg, App};
use std::io::{self, Read};
use std::fs::File;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("file").value_name("FILE").index(1))
        .get_matches();

    let mut buffer = String::new();
    if atty::is(Stream::Stdin) {
        match matches.value_of("file") {
            Some(file_name) => {
                let mut f = File::open(file_name).expect("file not found");
                f.read_to_string(&mut buffer).expect(
                    "something went wrong reading the file",
                );
            }
            _ => return,
        }
    } else {
        io::stdin().read_to_string(&mut buffer).expect(
            "can't read.",
        );
    }
    match gitconfig2json::run(buffer.as_ref()) {
        Ok(json) => println!("{}", json),
        Err(err) => eprintln!("{}", err),
    }
}
