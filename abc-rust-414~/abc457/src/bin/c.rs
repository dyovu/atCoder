// use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let line: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let n = line[0];
    let k = line[1];

    let mut arr_2d: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n{
        let arr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        arr_2d.push(arr);
    }

    let c: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut idx = 0;
    let mut sum = 0;
    loop {
        if k <= sum + c[idx] * (arr_2d[idx].len() - 1){
            break
        }
        sum += c[idx] * (arr_2d[idx].len() - 1);
        idx += 1;
    }
    let ans_idx = (k - sum - 1) % (arr_2d[idx].len() - 1) + 1;
    // println!("{}, {}, {}", idx, ans_idx, sum);
    println!("{}", arr_2d[idx][ans_idx]);
}
