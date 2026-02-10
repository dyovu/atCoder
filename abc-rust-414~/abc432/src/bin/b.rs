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

    let mut vec = n.chars().map(|x| x.to_digit(10).unwrap() as u32).collect::<Vec<u32>>();

    vec.sort();

    let mut zero_count = 0;
    let mut ans = String::new();

    for i in vec.iter(){
        if *i == 0{
            zero_count += 1;
            continue;
        }
        ans.push(char::from_digit(*i, 10).unwrap());
    }



    println!("{}{}{}", &vec[zero_count], "0".repeat(zero_count), &ans[1..]);

}