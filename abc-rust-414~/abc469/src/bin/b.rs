use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        s: String,
    }

    let seats: Vec<char> = std::iter::once('x')
        .chain(s.chars())
        .chain(std::iter::once('x'))
        .collect();
    let mut cnt = 0;
    for i in 1..=n{
        if seats[i - 1] == 'x' && seats[i] == 'x' && seats[i + 1] == 'x'{
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
