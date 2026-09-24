use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut a: [isize; n],
    }
    
    a.sort();
    let mut p_idx = a.partition_point(|&x| x < 0) as isize;
    let mut n_idx = p_idx - 1;
    let mut pos = 0;
    let mut ans = 0;
    loop{
        if p_idx == n as isize{
            ans += pos - a[0];
            break;
        }
        if -1 == n_idx{
            ans += a[n - 1] - pos;
            break;
        }
        let m = pos - a[n_idx as usize];
        let p = a[p_idx as usize] - pos;
        if m <= p{
            ans += m;
            pos = a[n_idx as usize];
            n_idx -= 1;
        }else{
            ans += p;
            pos = a[p_idx as usize];
            p_idx += 1;
        }
    }
    println!("{}", ans);
}
