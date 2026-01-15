use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String,
    }

    let mut vec: Vec<usize> = Vec::new();
    let first_block_is_t = s.chars().nth(0).unwrap() == 't';
    
    let mut current_is_t = first_block_is_t;
    let mut count = 0;

    for c in s.chars() {
        let char_is_t = c == 't';
        if char_is_t != current_is_t {
            vec.push(count);
            count = 1;
            current_is_t = char_is_t;
        } else {
            count += 1;
        }
    }
    vec.push(count);
    // println!("{:?}", vec);

    // 充填率の最大値
    let mut max_filling_rate: f64 = 0.0;
    // vecの長さ
    let n = vec.len();

    // vecの中のtのブロックのindex
    let t_indices: Vec<usize> = (0..n)
        .filter(|&i| {
            if first_block_is_t { i % 2 == 0 } else { i % 2 != 0 }
        })
        .collect();


    // println!("{:?}", t_indices);

    for i in &t_indices{
        if vec[*i] >= 3  {
            println!("{}", 1.0 as f64);
            return
        }
    }
    if t_indices.len() < 2{
        println!("{}", 0.0 as f64);
        return
    }

    for i in 0..t_indices.len()-1{
        let first_t_index = t_indices[i];
        let mut t_count:f64 = vec[first_t_index] as f64;
        let mut other_count:f64 = 0.0;

        for j in i+1..t_indices.len(){
            let t_index = t_indices[j];
            t_count += vec[t_index] as f64;
            other_count += vec[t_index-1] as f64;
            let filling_rate = (t_count - 2.0)/(t_count + other_count - 2.0);
            if max_filling_rate < filling_rate{
                max_filling_rate = filling_rate
            }
        }
    }


    println!("{}", max_filling_rate);

}