use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        mut x: usize,
        mut y: usize,
        z: usize,
    }

    let mut ans = "No";
    while z*y <= x{
        // println!("{}, {}", x, z*(y));
        if x == z*y{
            ans = "Yes";
        }
        x += 1;
        y += 1;

    }

    println!("{}", ans);



}