use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        x: isize,
        y: isize,
        l: isize,
        r: isize,
        a: isize,
        b: isize,
    }
    println!("{}", (l.min(b) - a).max(0) * y + (b.min(r) - a.max(l)).max(0) * x + (b - r.max(a)).max(0) * y);
}
