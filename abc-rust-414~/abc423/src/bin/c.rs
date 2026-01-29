use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        r: usize,
        a: [usize; n],
    }

    let mut left_open: usize = 0;
    let mut right_open: usize = 0;

    for i in 0..r{
        if a[i] == 0{
            left_open += 1;
        }else if a[i] == 1 && 0 < left_open{
            left_open += 2;
        }
    }

    for i in r..n{
        let index = r + n - i - 1;
        if a[index] == 0{
            right_open += 1;
        }else if a[index] == 1 && 0 < right_open{
            right_open += 2;
        }
    }

    // println!("{}, {}", left_open, right_open);

    println!("{}", left_open + right_open);

}