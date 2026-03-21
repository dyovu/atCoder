use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let mut lines = std::io::stdin().lock().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut vec: Vec<Vec<usize>> = Vec::new();

    for i in 0..n-1{
        let row:Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        vec.push(row);
    }

    // println!("{:?}", vec);

    for i in 0..n-2{
        for j in i+2..n{
            for k in i+1..j{
                let direct = vec[i][j-i-1];
                let via = vec[i][k-i-1] + vec[k][j-k-1];
                if via < direct {
                    println!("Yes");
                    return
                }
            }
        }
    }

    println!("No");

}