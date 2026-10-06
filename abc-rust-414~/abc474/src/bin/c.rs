use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        q: usize,
        p: [usize; n],
        a: [usize; q],
    }

    let mut vec = Vec::new();
    let mut set = HashSet::new();
    for i in a.iter().rev(){
        if !set.contains(i){
            vec.push(*i);
            set.insert(*i);
        }
    }
    for i in p.iter().rev(){
        if !set.contains(i){
            vec.push(*i);
            set.insert(*i);
        }
    }
    for (idx, val ) in vec.iter().rev().enumerate(){
        if idx != 0{
            println!(" ");
        }
        println!("{}", val);
    }
}
