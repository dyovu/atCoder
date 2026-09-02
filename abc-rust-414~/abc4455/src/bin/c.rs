use std::collections::HashMap;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut sum_num: HashMap<usize, usize> = HashMap::new();
    let mut sum = 0;

    for i in a.iter(){
        *sum_num.entry(*i).or_insert(0) += i;
        sum += i;
    }

    let mut vec: Vec<(&usize, &usize)> = sum_num.iter().collect();
    vec.sort_by(|a, b| (b.1).cmp(&a.1));
    // println!("{:?}", vec);
    
    for i in 0..k{
        let val = vec[i].1;
        sum -= val;
        if sum == 0{
            break
        }
    }
    println!("{}", sum);
}
