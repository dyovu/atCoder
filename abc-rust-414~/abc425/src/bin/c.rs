use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}


fn print_l_r(l: usize, r: usize, front_index: usize, cum: &Vec<usize>, n: usize) {
    // 論理的インデックスl, rを物理的インデックスに変換（0-indexed）
    let start = (front_index + l - 1) % n;
    let end = (front_index + r - 1) % n;
    
    let sum: usize;
    
    if start <= end {
        // 範囲が配列の終端をまたがない場合
        sum = cum[end + 1] - cum[start];
    } else {
        // 範囲が配列の終端をまたぐ場合
        sum = (cum[n] - cum[start]) + cum[end + 1];
    }
    
    println!("{}", sum);
}

fn main(){
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let first_line  = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let n = first_line[0];
    let q = first_line[1];

    let a: Vec<usize>  = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut cum: Vec<usize> = Vec::new();
    cum.push(0);
    let mut sum:usize = 0;
    for i in a{
        sum += i;
        cum.push(sum);
    }

    let mut front_index:usize = 0;

    for _ in 0..q{
        let line  = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let query_type = line[0];

        match query_type{
            1 => {
                let c = line[1];
                front_index = (front_index + c)%n;
            }
            2 =>{
                let l = line[1];
                let r = line[2];
                print_l_r(l, r, front_index, &cum, n);
            }
            _ => {}
        }
    }
}