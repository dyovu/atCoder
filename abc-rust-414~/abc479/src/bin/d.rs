use std::usize;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        q: usize,
        cost_l: [usize; n - 1],
        cost_r: [usize; n - 1],
        q: [[usize; 2]; q],
    }

    let mut l_min: Vec<usize> = vec![0; n - 1];
    let mut r_min: Vec<usize> = vec![0; n - 1];
    
    let mut current_min = usize::MAX;
    for (idx, &val) in cost_l.iter().enumerate().rev(){
        if val < current_min{
            current_min = val;
        }
        l_min[idx] = current_min;
    }

    current_min = usize::MAX;
    for (idx, &val) in cost_r.iter().enumerate(){
        if val < current_min{
            current_min = val;
        }
        r_min[idx] = current_min;
    }
    
    // println!("cost_l: {:?}, cost_r: {:?}", cost_l, cost_r);
    // println!("l_min: {:?}, r_min: {:?}", l_min, r_min);

    for i in q{
        let s = i[0];
        let g = i[1];
        if s < g{
            if s == 1{
                println!("{}", cost_r[s - 1]);
                continue;
            }
            println!("{}", cost_r[s - 1].min(cost_l[s - 2] + r_min[s - 1]));
        }else{
            if s == n{
                println!("{}", cost_l[s - 2]);
                continue;
            }
            println!("{}", cost_l[s - 2].min(cost_r[s - 1] + l_min[s - 2]));
        }
    }
}
