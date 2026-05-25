use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        k: usize,
    }

    let mut cnt = 0;
    for i in 0..=n{
        let s = i.to_string();
        let mut sum_digits: usize = 0;
        for j in s.chars(){
            sum_digits += j.to_digit(10).unwrap() as usize;
            // println!("{}", j);
        }
        if sum_digits == k{
            cnt += 1;
        }
    }

    println!("{}", cnt);

}