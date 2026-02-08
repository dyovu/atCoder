use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: usize,
        b: usize,
        s: String,
    }

    let mut cum_a:Vec<usize> = vec![0];
    let mut cum_b:Vec<usize> = vec![0];

    for i in s.chars(){
        if i == 'a'{
            cum_a.push(*cum_a.last().unwrap() + 1);
            cum_b.push(*cum_b.last().unwrap());
        }else{
            cum_a.push(*cum_a.last().unwrap());
            cum_b.push(*cum_b.last().unwrap() + 1)
        }
    }

    // println!("cum_a: {:?}", cum_a);
    // println!("cum_b: {:?}", cum_b);

    let mut ans:usize = 0;

    for i in 0..n{
        let a_under_i = cum_a[i];
        let b_under_i = cum_b[i];

        let min_pos_a = cum_a.partition_point(|&x| x < a+a_under_i) - 1;
        let min_pos_b = cum_b.partition_point(|&x| x < b+b_under_i) - 2;

        // println!("i={}", i);
        // println!("a_under_i={}, min_pos_a={}, b_under_i={}, min_pos_b={}",a_under_i, min_pos_a, b_under_i, min_pos_b);

        if n <= min_pos_a ||  min_pos_b < min_pos_a{
            continue;
        }
        // println!("i={}", i);
        // println!("a_under_i={}, min_pos_a={}, b_under_i={}, min_pos_b={}",a_under_i, min_pos_a, b_under_i, min_pos_b);

        ans += (min_pos_b - min_pos_a +1);
    }

    println!("{}", ans);





}