use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin(); 
    let mut lines = stdin.lock().lines();
    let tmp = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>(); let q = tmp[1];

    let mut vec = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut sorted = vec.iter().copied().collect::<Vec<usize>>();
    sorted.sort();
    println!("{:?}", vec);
    for _ in 0..q{
        let k: usize= lines.next().unwrap().unwrap().parse().unwrap();
        let mut sub_vec= lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| vec[x.parse::<usize>().unwrap() - 1])
        .collect::<Vec<usize>>();
        sub_vec.sort();

        println!("{:?}", sub_vec);
        let mut idx = 0;
        for i in sub_vec{
           if sorted[idx] != i{
                break        
            }
            idx += 1;
        }
        println!("{}", sorted[idx])
    }
}
