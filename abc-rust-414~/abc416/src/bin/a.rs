use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        _: usize,
        l: usize,
        r: usize,
        s: String,
    }

    let s_vec: Vec<char> = s.chars().collect();

    let mut ans = "Yes";
    for i in l-1..r{
        if s_vec[i] == 'x'{
            ans = "No";
        }
    }

    println!("{}", ans);
}
