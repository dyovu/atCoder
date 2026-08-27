use proconio::input;
// use std::{collections::{BTreeMap, HashMap}, io::{self, BufRead}};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String,
    }
    
   let mut map = BTreeMap::new();

    for i in s.chars(){
       *map.entry(i).or_insert(0) += 1;
    }
    let max = map.values().max().unwrap();
    for i in s.chars(){
        if map.get(&i).unwrap() != max{
            print!("{}", i);
        }
    }
}

