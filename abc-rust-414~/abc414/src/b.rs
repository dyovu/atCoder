use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main_b(){
    input!{
        n: usize,
        a: [[String; 2]; n],
    }

    let mut result = String::new();
    let mut total_number = 0;
    for i in a{
        let char = i[0].clone();
        let num: usize = i[1].parse().unwrap();
        total_number += num;
        if total_number > 100{
            result = "Too Long".to_string();
            break;
        }
        for _ in 0..num{
            result.push_str(&char);
        }
    }
    print!("{}", result);
}