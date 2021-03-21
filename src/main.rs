use mactags::{Tags, TAG_ATTR_KEY};
use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let mut args = env::args().skip(1);
    args.next()
        .map_or_else(print_help, |first| match &first[..] {
            "get" | "g" => {
                args.next()
                    .map_or_else(|| eprintln!("Usage: mactags get FILE"), get);
            }
            "remove" | "rm" | "r" => args.next().map_or_else(
                || eprintln!("Usage: mactags remove FILE [TAGS...]"),
                |f| remove(&f, args.collect()),
            ),
            "add" | "a" => args.next().map_or_else(
                || eprintln!("Usage: mactags add FILE [TAGS...]"),
                |f| add(&f, args.collect()),
            ),
            "update" | "u" => args.next().map_or_else(
                || eprintln!("Usage: mactags update FILE [TAGS...]"),
                |f| update(&f, args.collect()),
            ),
            f => get(f),
        });
}

pub fn get<S: AsRef<str>>(f: S) {
    let f = f.as_ref();

    for tag in Tags::from_path(f).parse() {
        println!("{}", tag);
    }
}

pub fn remove(f: &str, args: Vec<String>) {
    panic!("Not implemented yet!")
    // let tags = Tags::from_path(f);
    // for tag in &args {}
}

pub fn add(f: &str, args: Vec<String>) {
    panic!("Not implemented yet!")
}

pub fn update(f: &str, args: Vec<String>) {
    panic!("Not implemented yet!")
    // let tag = Tags::from_tags(args);
    // xattr::set(f, TAG_ATTR_KEY, &tag.data).unwrap();
}

fn print_help() {
    eprintln!("USAGE: mactags [get|remove|add|update] FILE [TAGS...]");
}
