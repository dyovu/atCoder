use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [isize; n],
        b: [isize; n],
        c: [isize; n],
    }

    let mut cum_a:Vec<isize> = vec![0];
    let mut cum_b:Vec<isize> = vec![0];
    let mut cum_c:Vec<isize> = vec![0];

    for i in 0..n{
        cum_a.push(cum_a[i] + a[i]);
        cum_b.push(cum_b[i] + b[i]);
        cum_c.push(cum_c[i] + c[i]);
    }

    let mut cum_g:Vec<isize> = Vec::new();
    let mut cum_h:Vec<isize> = Vec::new();

    for i in 0..n+1{
        cum_g.push(cum_a[i] - cum_b[i]);
        cum_h.push(cum_b[i] - cum_c[i]);
    }

    // println!("{:?}", cum_g);

    let mut max_u_cum_g:isize = cum_g[1];

    let mut ans:isize = isize::MIN;
    for i in 1..n-1{
        if max_u_cum_g < cum_g[i]{
            max_u_cum_g = cum_g[i];
        }
        // println!("i: {}, sum: {}", i, max_u_cum_g + cum_h[i]);

        ans = ans.max(max_u_cum_g + cum_h[i+1]);
    }

    println!("{}", ans + cum_c[n]);

}