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

    let mut max = a[0];

    for (i, v) in a.iter().enumerate(){
        // println!("{}, {}", i, v);
        if  max <= *v {
            println!("-1");
            max = *v;
            continue;
        }

        for j in 0..i{
            if *v < a[i-j-1]{
                println!("{}", i-j);
                break;
            }
        }
    }

}