use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: String,
    }
    let n_vec: Vec<char> = n.chars().collect();
    let mut ans: Vec<usize> = Vec::new();

    for (index, value) in n_vec.iter().enumerate(){
        if value == &'#' {
            ans.push (index+1);
        }
    }

    for i in 0..ans.len()/2{
        println!("{},{}", ans[i*2], ans[i*2+1])
    }
}