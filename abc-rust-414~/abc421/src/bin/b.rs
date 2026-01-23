use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        x: usize,
        y: usize,
    }

    let mut vec: Vec<usize> = Vec::new();
    vec.push(x);
    vec.push(y);

    // println!("{:?}", vec);

    for i in 2..10{
        let a = vec[i-1] + vec[i-2];
        // println!("a {}", a);

        let a_str = a.to_string();
        let a_inv = a_str.chars().rev().collect::<String>().parse::<usize>().unwrap();
        // println!("a_inv {}", a_inv);
        vec.push(a_inv);
    }

    println!("{}", vec[9]);

}