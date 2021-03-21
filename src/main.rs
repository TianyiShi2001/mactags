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

    for tag in Tags::from_path(f)
        .map(|tags| tags.parse())
        .unwrap_or_default()
    {
        println!("{}", tag);
    }
}

pub fn remove_all(f: &str) {
    xattr::remove(f, TAG_ATTR_KEY).unwrap();
}

pub fn remove(f: &str, mut args: Vec<String>) {
    if args.is_empty() {
        remove_all(f);
        return;
    }
    let tags = Tags::from_path(f)
        .map(|tags| tags.parse())
        .unwrap_or_default();
    let tags = tags
        .into_iter()
        .filter(|t| {
            if let Some(i) = args.iter().position(|u| t == u) {
                args.remove(i);
                false
            } else {
                true
            }
        })
        .collect();
    update(f, tags);
}

pub fn add(f: &str, args: Vec<String>) {
    if args.is_empty() {
        panic!("Usage: mactags add FILE TAGS...");
    }
    let mut tags = Tags::from_path(f)
        .map(|tags| tags.parse())
        .unwrap_or_default();
    for tag in args {
        if tags.iter().all(|t| t != &tag) {
            tags.push(tag);
        }
    }
    update(f, tags);
}

pub fn update(f: &str, args: Vec<String>) {
    if args.is_empty() {
        panic!("Usage: mactags update FILE TAGS...; use `mactags remove` to remove all tags.")
    }
    let tag = Tags::from_tags(args);
    xattr::set(f, TAG_ATTR_KEY, &tag.data).unwrap();
}

fn print_help() {
    eprintln!("USAGE: mactags [get|remove|add|update] FILE [TAGS...]");
}
