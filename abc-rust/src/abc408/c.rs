use proconio::input;
use std::collections::*;


fn main(){
    input!{
        n :i32,
        m :i32,
        lr : [[i32;2];m],
    }
    
    // println!("{:?}", lr);
    
    let mut walls = BTreeMap::new();
    for i in 1..=n+1{
      walls.insert(i, 0);
    }

    for i in lr{
        let count_inc = walls.entry(i[0]).or_insert(0);
        *count_inc += 1;
        let count_dec = walls.entry(i[1]+1).or_insert(0);
        *count_dec -= 1;
    }

    let mut min: i32 = 1e7 as i32;
    let mut current_battery:i32 = 0;
    for i in 1..=n{
        if let Some(&v) = walls.get(&i) {
            current_battery += v;
        }
        min = min.min(current_battery);
    }

    println!("{}", min);
    // println!("{:?}", walls)
}