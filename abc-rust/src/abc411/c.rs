use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        n : usize,
        q : usize,
        a : [usize; q]
    };

    let mut tile:Vec<usize> = vec![0; n+2];

    let mut count = 0;
    for i in a{
        let target = tile[i];
        let prev = tile[i-1];
        let next = tile[i+1];
        if target == 0{
            if prev == 0 && next == 0{
                count += 1;
            }else if prev == 1 && next == 1{
                count -= 1;
            }
            tile[i] = 1
        }else{
            if prev == 0 && next == 0{
                count -= 1;
            }else if prev == 1 && next == 1{
                count += 1;
            }
            tile[i] = 0
        }
        println!("{}", count);
    }


}