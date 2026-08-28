use proconio::input;
// use std::io::{self, BufRead};
use std::{collections::*, hash::Hash, panic::UnwindSafe};

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        l: usize,
        r: usize,
        s: String,
    }
    let mut count: HashMap<char, usize> = HashMap::new();
    let mut cum_sum: Vec<HashMap<char, usize>> = vec![HashMap::new(); n];

    for (i, c) in s.chars().enumerate(){
        *count.entry(c).or_insert(0) += 1;
        cum_sum[i] = count.clone();
    }

    let mut ans = 0;
    let mut iter = s.chars();
    for i in 0..n - l - 1{
        let limit = (n - 1).min(i + r);
        let c = iter.next().unwrap();
        ans +=  cum_sum[limit][&c] - cum_sum[i + l -1][&c];
    }
    println!("{}", ans);

}
