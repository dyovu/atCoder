use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        q: usize,
        s: String,
        t: String,
        query: [[usize; 2]; q],
    }

    let mut vec = Vec::new();
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();
    let t_len = t_vec.len();
    let s_len = s_vec.len();
    if t_len <= s_len{
        for i in 0..s_len - t_len + 1{
            let mut flag = true;
            for j in 0..t_len{
                if s_vec[i + j] != t_vec[j]{
                    flag = false;
                    break;
                }
            }
            if flag{
                vec.push((i + 1, i + t_len));
            }
        }
    }
    vec.sort_by(|&a, &b| a.0.cmp(&b.0));
    // println!("{:?}", vec);

    for i in query{
        let l = i[0];
        let r = i[1];
        let point = vec.partition_point(|&x| x.0 < l);
        if point == vec.len(){
            println!("No");
            continue;
        }
        if vec[point].1 <= r{
            println!("Yes");
        }else{
            println!("No");
        }
    }
}

