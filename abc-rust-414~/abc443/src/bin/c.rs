use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        t: usize,
        a: [usize; n],
    }

    let mut start:usize = 0;
    let mut cnt:usize = 0;

    for i in a{
        if start < i{
            cnt += i-start;
            start = i+100;
        }
    }

    if start < t{
        cnt += t-start;
    }

    println!("{}", cnt);

}