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

    let mut prev_idx = 0;
    let mut back_idx = n - 1;
    while prev_idx < n - 1 && a[prev_idx] <= a[prev_idx + 1]{
        prev_idx += 1;
    }
    while 0 < back_idx && a[back_idx - 1] <= a[back_idx]{
        back_idx -= 1;
    }

    // println!("{}, {}", prev_idx, back_idx);
    if back_idx < prev_idx{
        println!("Yes");
        std::process::exit(0);
    } else if k <= back_idx - prev_idx{
        println!("No");
        std::process::exit(0);
    }
    let mut slice: Vec<usize> = a[prev_idx..=back_idx].to_vec();
    slice.sort();
    // println!("{:?}", slice);

    let mut cnt = 0;
    while prev_idx == 0 || a[prev_idx - 1] > slice[0]{
        prev_idx -= 1;
        cnt += 1;
    }
    while back_idx == n - 1 || a[back_idx + 1] < slice[slice.len() - 1]{
        back_idx += 1;
        cnt += 1;
    }
    // println!("{}", cnt);
    if cnt + slice.len() <= k{
        println!("Yes");
    }else{
        println!("No");
    }
}

