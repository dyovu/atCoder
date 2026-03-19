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

    let mut map:BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for (index, &value) in a.iter().enumerate(){
        map.entry(value).or_insert(Vec::new()).push(index+1);
    }

    // println!("{:?}", map);
    let mut ans = 0;

    for (ak, index) in map.iter(){
        if ak%3 != 0 { continue }

        let mut ak_index = index.clone();
        ak_index.sort();

        let aj = ak *5/3;
        let ai = ak *7/3;

        let aj_index = match map.get(&aj){
            Some(t) => t,
            None => {continue}
        };

        let mut ai_index = match map.get(&ai){
            Some(t) => t.clone(),
            None => {continue}
        };
        ai_index.sort();

        // println!("ak_index: {:?}, aj_index, {:?}, ai_index: {:?},", ak_index, aj_index, ai_index);

        for j in aj_index{
            let ak_under_j = ak_index.partition_point(|x| x < j);
            let ai_under_j = ai_index.partition_point(|x| x < j);

            // println!("ak_under_j: {}, ai_under_j: {}", ak_under_j, ai_under_j);

            ans += ak_under_j*ai_under_j;

            let ak_over_j = ak_index.len() - ak_under_j;
            let ai_over_j = ai_index.len() - ai_under_j;

            // println!("ak_over_j: {}, ai_over_j: {}", ak_over_j, ai_over_j);

            ans += ak_over_j*ai_over_j;
        }
        // println!("ak: {}, ans: {}", ak, ans);
        
    }

    println!("{}", ans);
    
}   