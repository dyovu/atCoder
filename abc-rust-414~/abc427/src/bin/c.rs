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
        a: [[usize; 2]; m],
    }

    let ten = 0b0000000000;
    // println!("{:04b}", ten^0b001);

    let mut min_del = m;

    for bit in 0..(1 << n){
        let mut count = 0;
        // println!("i : {}, bit :{:4b}", bit, bit);


        for j in a.iter(){
            let u = j[0];
            let v = j[1];

            // println!("u = {}, (bit >> u) & 1 :{:4b}, v = {}, (bit >> v) & 1 :{:4b} ", u, bit >> u, v, bit >> v);
            if (bit >> u) & 1 == (bit >> v) & 1{
                count += 1;
            }
        }

        // println!("i : {}, bit :{:4b}, count :{}", bit, bit, count);


        if count < min_del {min_del = count}
    }

    println!("{}", min_del);


}