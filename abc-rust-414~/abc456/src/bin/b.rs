use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        alpha: [usize; 6],
        beta: [usize; 6],
        gamma: [usize; 6],
    }

    let mut cnt = 0;
    for i in 0..3{
        let cnt_a = alpha.iter().filter(|&&x| x == i + 4).count();
        
        let cnt_b_1 = beta.iter().filter(|&&x| x == (i + 1) % 3 + 4).count();
        let cnt_c_1 = gamma.iter().filter(|&&x| x == (i + 2) % 3 + 4).count();
        
        let cnt_b_2 = beta.iter().filter(|&&x| x == (i + 2) % 3 + 4).count();
        let cnt_c_2 = gamma.iter().filter(|&&x| x == (i + 1) % 3 + 4).count();

        cnt += cnt_a * (cnt_b_1 * cnt_c_1 + cnt_c_2 * cnt_b_2);
    }

//    println!("{}", cnt);
    println!("{}", cnt as f64 / 216.0);

}
