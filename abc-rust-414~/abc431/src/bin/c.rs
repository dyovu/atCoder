use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        k: usize,
        mut h: [usize; n],
        mut b: [usize; m],
    }

    h.sort();
    b.sort();

    if n < k || m < k{
        println!("No");
        return
    }

    let mut h_iter = h.iter();
    let mut b_iter = b.iter();
    let mut count:usize = 0;

    'outer: for h_w in h_iter{

        let mut b_w:&usize = match b_iter.next(){
            Some(x) => x,
            None => {
                break 'outer
            }
        };
        // println!("h_w={}, b_w={}",h_w, b_w);
        // println!("{}", count);

        while b_w < h_w{
            b_w = match b_iter.next(){
                Some(x) => x,
                None => {
                    break 'outer
                }
            };
            // println!("h_w={}, b_w={}",h_w, b_w);
        }
        count += 1;
    }

    // println!("count={}", count);

    if k <= count{
        println!("Yes")
    }else{
        println!("No")
    }

}