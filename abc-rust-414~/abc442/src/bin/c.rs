use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn comb(n: usize, r: usize) -> usize {
    (0..r).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [[usize; 2]; m],
    }

    let mut relation: Vec<usize> = vec![0; n];

    for i in a{
        relation[i[0]-1] += 1;
        relation[i[1]-1] += 1;
    }

    // println!("{:?}", relation);

    for i in relation{
        if (n - i - 1) < 3{
            print!("{} ", 0);
            continue
        }

        print!("{} ", comb(n - i - 1, 3));
    }


}