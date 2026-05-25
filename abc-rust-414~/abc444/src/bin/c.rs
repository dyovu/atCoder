use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    let len_a = a.len();
    let mut ans  = BTreeSet::new();

    let last = a.last().unwrap();
    let len = a.partition_point(|&x| x < *last);
    let slice = &a[..len];

    let sum_l = a[0] + a[len_a-1];
    ans.insert(*last);
    ans.insert(sum_l);
    // println!("{:?}", ans);

    for i in 0..len{
        if i == len -i -1{
            ans.remove(last);
            break
        }

        let sum = slice[i] + slice[len -i -1];
        if &sum != last{
            ans.remove(last);
            break
        }
    }

    for (i, v) in a.iter().enumerate(){
        if i == len_a -i -1{
            ans.remove(&sum_l);
            break
        }
        if sum_l != v + &a[len_a -i -1]{
            ans.remove(&sum_l);
            break
        }
    }

    ans.iter().for_each(|x| print!("{} ", x));
}