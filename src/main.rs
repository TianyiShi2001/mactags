use mactags::Tags;
use std::env;
use xattr;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let mut args = env::args().skip(1);
    args.next()
        .map_or_else(print_help, |first| match &first[..] {
            "get" | "g" => {
                args.next()
                    .map_or_else(|| eprintln!("Usage: mactag get FILENAME"), get);
            }
            "remove" | "rm" | "r" => args.next().map_or_else(
                || eprintln!("Usage: mactag remove [TAGS...]"),
                |f| remove(&f, args.collect()),
            ),
            "add" | "a" => args.next().map_or_else(
                || eprintln!("Usage: mactag add [TAGS...]"),
                |f| add(&f, args.collect()),
            ),
            "update" | "u" => args.next().map_or_else(
                || eprintln!("Usage: mactag update [TAGS...]"),
                |f| add(&f, args.collect()),
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
}

fn print_help() {
    eprintln!("USAGE: mactag [get|remove|add|update] FILE [TAGS...]");
}
