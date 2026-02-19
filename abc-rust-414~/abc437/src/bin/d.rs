use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: isize,
        m: usize,
        mut a: [isize; n],
        mut b: [isize; m],
    }
    const DIVIDER:isize = 998244353;

    a.sort();
    b.sort();

    let mut cum_sum_b = vec![0];
    for i  in 0..m{
        cum_sum_b.push(cum_sum_b[i] + b[i]);
    }

    // println!("a: {:?}, b: {:?}, cum_sum_b: {:?}",a, b, cum_sum_b);

    let ans:isize = a
        .iter()
        .map(|x| {
            let lowe_bound = b.partition_point(|y| y < x);
            let positive = x*(lowe_bound as isize) - cum_sum_b[lowe_bound];
            let negative = cum_sum_b[m] - cum_sum_b[lowe_bound] - x*(m - lowe_bound) as isize;
            // println!("{}, {}", positive, negative);
            (positive + negative)%DIVIDER
        })
        .sum();
    

    println!("{}",ans%DIVIDER);



}