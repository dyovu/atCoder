use std::io::BufRead;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let first_linep: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let n: usize = first_linep[0];
    let q: usize = first_linep[1];

    let mut vec = vec![0; n];
    let mut natu: Vec<usize> = Vec::new();
    let mut bit_xor = 0;

    for line in lines.take(q){
        let query:Vec<usize> = line.unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        match query[0]{
            1 => {
                let idx = query[1] - 1;
                if vec[idx] == 0{
                    natu.push(idx);
                }
                bit_xor = bit_xor ^ vec[idx] ^ (vec[idx] + 1);
                vec[idx] += 1;
            }
            2 => {
                for &i in natu.iter(){
                    bit_xor = bit_xor ^ vec[i] ^ (vec[i] - 1);
                    vec[i] -= 1;
                }
                natu = natu.iter().filter(|&&x | 0 < vec[x]).copied().collect();
            }
            _ => {}
        }
        println!("{}", bit_xor);
    }
}
