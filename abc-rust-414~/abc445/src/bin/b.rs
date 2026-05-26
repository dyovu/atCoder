use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [String; n],
    }

    let m = a.iter().map(|s| s.len()).max().unwrap();

    for i in a{
        let diff = (m - i.len())/2;
        // println!("{}", diff);

        let fix = ".".repeat(diff);
        print!("{}", fix);
        print!("{}", i);
        println!("{}", fix);
    }

}