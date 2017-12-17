extern crate day12;

use std::io::prelude::*;
use day12::*;

fn read_all<S: AsRef<std::path::Path>>(path: S) -> String {
    let mut content = String::new();
    let mut f = std::fs::File::open(path).unwrap();
    f.read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let fname = std::env::args().nth(1).unwrap_or(String::from("example"));
    let content = read_all(fname);

    let sets: Sets = content.parse::<Configurations>().unwrap().into();

    println!("Set[0].size = {}", sets.get(0).size);

    let roots : std::collections::HashSet<_> = sets.map.keys().map(|&pid| sets.root(pid)).collect();

    println!("Gropus = {}", roots.len());
}
