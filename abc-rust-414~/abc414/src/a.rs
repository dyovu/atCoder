use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn main_a(){
    input!{
        n: usize,
        l: usize,
        r: usize,
        a: [[usize; 2]; n],
    }

    let mut count = 0;
    // print_type(&a[0]);
    for i in a{
        if i[0] <= l && r <= i[1]{
            count += 1;
        }
    }

    print!("{}", count);

}

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
    let (_, max) = map.iter().max().unwrap();
    for i in s.chars(){
        if map.get(&i).unwrap() != max{
            print!("{}", i);
        }
    }
}

