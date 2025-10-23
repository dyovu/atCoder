use proconio::input;
// use std::collections::*;

fn main(){
    input!{
        n : usize,
        l : usize,
        a : [usize; n-1],
    }

    if l%3 != 0{
        print!("{}", 0);
        std::process::exit(0);
    }
    let distance = l/3;

    let mut count = vec![0; l as usize];
    count[0] = 1;

    let mut current_position = 0;
    for i in a{
        current_position = (current_position+i)%l;
        count[current_position as usize] += 1;
    }

    let mut ans:usize = 0;
    for i in 0..distance{
        let c1 = count[i as usize];
        let c2 = count[(i + distance) as usize];
        let c3 = count[(i + distance*2) as usize];

        ans += c1 * c2 * c3;
        // println!("i : {} {}", i ,ans);
    }
    println!("{}", ans);
}