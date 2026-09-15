use std::collections::HashMap;

use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [[usize; 2]; n],
    }
    
    let mut map = HashMap::new();
    for i in a{
        let mut entry = map.entry(i[0]).or_insert(0);
        if *entry < i[1]{
            *entry = i[1];
        }
    }
    for i in 1..=m{
        if i != 1{
            print!(" ");
        }
        match map.get(&i){
            Some(w) => {
                print!("{}", w);
            }
            None => {
                print!("-1");
            }
        }
    }
}
