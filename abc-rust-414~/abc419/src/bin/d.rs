use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn flip_bits(num: u32, l: usize, r: usize) -> u32 {
    let mask = ((1 << (r - l + 1)) - 1) << l;
    num ^ mask
}


fn main(){
    input!{
        n: usize,
        m: usize,
        mut s: String,
        mut t: String,
        a: [[usize; 2]; m],
    }

    let mut s_vec: Vec<char> = s.chars().collect();
    let mut t_vec: Vec<char> = t.chars().collect();

    for i in a.iter(){
        let l = i[0] - 1;
        let r = i[1] - 1;
        for j in l..=r {
            (s_vec[j], t_vec[j]) = (t_vec[j], s_vec[j]);
        }
    }

    let ans: String = s_vec.iter().collect();
    println!("{}", ans);
}