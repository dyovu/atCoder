use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }

    // println!("{:?}", a);

    let mut start: usize = 0;
    let mut end: usize = 0;

    for (index, value) in a.iter().enumerate(){
        // println!("{}, {}", index, value);
        if *value == 1 {
            start = index;
            break
        }
    }

    if start == a.len()-1{
        println!("0");
        return
    }

    for (index, value) in a.iter().rev().enumerate(){
        // println!("{}, {}", index, value);
        if *value == 1 {
            end = n - index -1;
            break
        }
    }

    println!("{}", end-start);

}