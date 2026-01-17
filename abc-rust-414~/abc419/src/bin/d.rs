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

    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    let mut switch_point:Vec<usize> = vec![0; n+1];

    // println!("{:?}", switch_point);

    for i in a.iter(){
        let l = i[0];
        let r = i[1];
        
        switch_point[l-1] = (switch_point[l-1]+1)%2;
        switch_point[r] = (switch_point[r]+1)%2;
        // println!("{:?}, l: {}, r: {}", switch_point, l, r);
        
    }

    // println!("{:?}", switch_point);

    let mut is_s = true;
    for (index, value) in switch_point[..n].iter().enumerate(){
        if value == &1{
            is_s = !is_s;   
        }
        match is_s{
            true => {print!("{}", s_vec[index])}
            false => {print!("{}", t_vec[index])}
        }
    }   

}