extern crate serde_json;

use std::error::Error;
use std::iter::FromIterator;
use self::serde_json::{Value, Map};

pub fn run(message: &str) -> Result<String, Box<Error>> {
    let git_configs = Vec::from_iter(message.split("\0").map(String::from));
    let mut map = Map::new();

    for git_config in &git_configs {
        if git_config.is_empty() {
            continue;
        }
        let (keys, value) = split_once(git_config);
        if keys.len() == 0 {
            continue;
        }
        let split_keys = Vec::from_iter(keys.split(".").map(String::from));
        match split_keys.len() {
            1 => {
                map.insert(split_keys[0].to_owned(), Value::String(value.to_owned()));
                ()
            }
            2 => {
                // TODO: reduce clone
                let cloned = map.clone();
                match cloned.get(&split_keys[0]) {
                    Some(object) => {
                        // TODO: reduce clone
                        let mut internal = object.as_object().unwrap().clone();
                        internal.insert(split_keys[1].to_owned(), Value::String(value.to_owned()));
                        map.insert(split_keys[0].to_owned(), Value::Object(internal));
                        ()
                    }
                    None => {
                        let mut internal = Map::new();
                        internal.insert(split_keys[1].to_owned(), Value::String(value.to_owned()));
                        map.insert(split_keys[0].to_owned(), Value::Object(internal));
                        ()
                    }
                }
            }
            n if n >= 3 => {
                // TODO: reduce clone
                let cloned_for_external = map.clone();
                match cloned_for_external.get(&split_keys[0]) {
                    Some(object) => {
                        // TODO: reduce clone
                        let mut external = object.as_object().unwrap().clone();
                        let cloned_external = external.clone();
                        match cloned_external.get(&split_keys[1..n - 1].join(".")) {
                            Some(object2) => {
                                // TODO: reduce clone
                                let mut internal = object2.as_object().unwrap().clone();
                                internal.insert(
                                    split_keys[n - 1].to_owned(),
                                    Value::String(value.to_owned()),
                                );
                                external.insert(
                                    split_keys[1..n - 1].join("."),
                                    Value::Object(internal),
                                );
                                map.insert(split_keys[0].to_owned(), Value::Object(external));
                                ()
                            }
                            None => {
                                let mut internal = Map::new();
                                internal.insert(
                                    split_keys[n - 1].to_owned(),
                                    Value::String(value.to_owned()),
                                );
                                external.insert(
                                    split_keys[1..n - 1].join("."),
                                    Value::Object(internal),
                                );
                                map.insert(split_keys[0].to_owned(), Value::Object(external));
                                ()
                            }
                        }
                    }
                    None => {
                        let mut internal = Map::new();
                        internal.insert(
                            split_keys[n - 1].to_owned(),
                            Value::String(value.to_owned()),
                        );
                        let mut external = Map::new();
                        external.insert(split_keys[1..n - 1].join("."), Value::Object(internal));
                        map.insert(split_keys[0].to_owned(), Value::Object(external));
                        ()
                    }
                }
            }
            _ => return Err(From::from("unexpected something happens.".to_owned())),
        }
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
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn parse() {
        let mut f = File::open("git-config-list-null.txt").expect("file not found");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect(
            "something went wrong reading the file",
        );
        println!("{}", buf);
        println!("----");
        println!("{:?}", run(buf.as_ref()).unwrap());
    }
}
