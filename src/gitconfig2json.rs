extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use self::serde_json::{Value, Map};

pub fn run(message: &str) -> Result<String, Box<Error>> {
    let git_configs = Vec::from_iter(message.split("\0").map(String::from));
    let mut map = Map::new();

    for git_config in &git_configs {
        let (keys, value) = split_once(git_config);
        if keys.len() == 0 {
            continue;
        }
        map.insert(keys.to_owned(), Value::String(value.to_owned()));
//        println!("{} --- {}", keys, value);
    }

    Ok(serde_json::to_string(&map).unwrap())
}

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, "\n");
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let mut f = File::open("git-config-list-null.txt").expect("file not found");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect(
            "something went wrong reading the file",
        );
        println!("{}", run(buf.as_ref()).unwrap());
    }
}
