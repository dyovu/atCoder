// use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for i in 0..q{
        let v: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let (n, d) = (v[0], v[1]);
        let a: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let b: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();

        let cumsum_a: Vec<usize> = a.iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect();

        let mut consumption = 0;
        for i in 0..n{
            consumption += b[i];
            if d <= i{
                if consumption < cumsum_a[i-d]{
                    consumption = cumsum_a[i-d]
                }
            }
        }

        println!("{}", cumsum_a.last().unwrap() - consumption);
    }
}