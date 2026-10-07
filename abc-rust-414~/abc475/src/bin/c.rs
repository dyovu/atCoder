use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        s: usize, 
        l: usize,
        a: [usize; n - 1],
    }

    let mut prev = Vec::new();
    prev.push(0);
    for i in 0..s - 1{
        if s as isize - 2 < 0{
            break;
        }
        prev.push(prev[i] + a[s - 2 - i]);
    }

    let mut nex = Vec::new();
    nex.push(0);
    for (i, val) in a[s - 1..n - 1].iter().enumerate(){
        nex.push(nex[i] + val);

    }
    // println!("{:?}", prev);
    // println!("{:?}", nex);
    
    let t = prev.partition_point(|&x| x <= l);
    let mut ans = t - 1;
    for i in (0..t).rev(){
        let p = nex.partition_point(|&x| 2 * x <= l - prev[i]);
        ans = ans.max(i + p - 1);
    }
    let t = nex.partition_point(|&x| x <= l);
    for i in (0..t).rev(){
        let p = prev.partition_point(|&x| 2 * x <= l - nex[i]);
        ans = ans.max(i + p - 1);
    }
    println!("{}", ans + 1);
}
