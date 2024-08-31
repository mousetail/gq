use std::fs::OpenOptions;

use gq::Builtin as GqBuiltin;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Builtin {
    name: String,
    description: String,
    symbol: char,
    template: String,
    brackets: Vec<BracketSpec>,
}

#[derive(Deserialize, Debug)]
struct BracketSpec {
    template: String,
    output: String,
}

fn main() {
    let reader = OpenOptions::new()
        .read(true)
        .open("templates/combinators.yaml")
        .unwrap();

    let value: Vec<Builtin> = serde_yml::from_reader(reader).unwrap();
    println!("{:#?}", value);
}
