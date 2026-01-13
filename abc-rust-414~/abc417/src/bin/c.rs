use proconio::input;
use std::collections::HashMap;

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    let mut count:i64 = 0;

    for j in 0..n{
        let target = j as i64 - a[j] as i64;
        count += map.get(&target).unwrap_or(&0);
        
        let key = j as i64 + a[j] as i64;
        *map.entry(key).or_insert(0) += 1;
    }

    println!("{}", count);
}