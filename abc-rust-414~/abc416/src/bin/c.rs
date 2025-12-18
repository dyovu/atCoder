use std::process::Command;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn product(ranges: &[usize]) -> Vec<Vec<usize>> {
    if ranges.is_empty() {
        return vec![vec![]];
    }
    
    let mut result = Vec::new();
    let first_range = ranges[0];
    let rest_product = product(&ranges[1..]);
    
    for i in 1..=first_range {
        for combination in &rest_product {
            let mut new_combination = vec![i];
            new_combination.extend(combination);
            result.push(new_combination);
        }
    }
    result
}

fn main(){
    input!{
        n: usize,
        k: usize,
        x: usize,
        a: [String; n],
    }

    let range = vec![n;k];
    let combinations = product(&range);
    // println!("{:?}", combinations);

    let mut str_arr: Vec<String> = Vec::new();

    for i in combinations{
        let mut tmp_str = String::new();
        for j in i{
            tmp_str.push_str(&a[j-1]);
        }
        str_arr.push(tmp_str);
    }

    str_arr.sort();
    println!("{}", str_arr[x-1]);
    
}