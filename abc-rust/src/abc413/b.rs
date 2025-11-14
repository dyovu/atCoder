use proconio::input;
use std::collections::*;

fn main(){
    input!{
        n: usize,
        s: [String; n],
    }

    let mut count = BTreeMap::new();

    for k1 in 0..n{
        for k2 in k1+1..n{
            let i = &s[k1];
            let j = &s[k2];
            let k = format!("{}{}", i, j);
            let l = format!("{}{}", j, i);
            let count_inc = count.entry(k).or_insert(0);
            *count_inc += 1;
            let count_inc = count.entry(l).or_insert(0);
            *count_inc += 1;
        }
    }

    // println!("{:?}", count);

    print!("{}", count.len());

}