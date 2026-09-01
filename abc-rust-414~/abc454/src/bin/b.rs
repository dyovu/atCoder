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
        cloth: [usize; n]
    }

    let mut q1_set = HashSet::new();
    
    for i in cloth{
        q1_set.insert(i);
    }

    if q1_set.len() == n { println!("Yes");} else {println!("No");}
    if q1_set.len() == m { println!("Yes");} else {println!("No");}
}
