// use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let v: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    let (n, _) = (v[0], v[1]);

    let mut ordered: HashSet<usize> = HashSet::new();

    for _ in 0..n{
        let _:usize = lines.next().unwrap().unwrap().parse().unwrap();
        let vec: Vec<usize>  = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();

        let mut ord = false;
        for i in vec{
            if !ordered.contains(&i){
                println!("{}", i);
                ordered.insert(i);
                ord = true;
                break;
            }
        }

        if !ord{
            println!("{}", 0);
        }
        
    }

}