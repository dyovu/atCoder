use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [String; n],
    }

    let mut map = BTreeMap::new();
    for i in a{
        *map.entry(i.to_uppercase()).or_insert(0) += 1;
    }
    let mut iter = map.values().max().unwrap();
    println!("{}", iter);
    // println!("{:?}", map);
}
