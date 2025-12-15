use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn n_number_palindrome(base:usize, mut n: usize) -> bool {
    let mut n_vec: Vec<usize> = Vec::new();

    while n > 0{
        let rem = n%base;
        n_vec.push(rem);
        n = n/base;
    }

    let len_n = n_vec.len();
    // println!("i:{}  n_vec: {:?}",n,  n_vec);

    let mut flag: bool = true;
    for i in 0..len_n/2 {
        if n_vec[i] != n_vec[len_n - 1 - i] {
            flag = false;
            break
        }
    }
    return flag;
}

// 与えられた上半分(切り上げ)の数から完全な回文の数を返す
fn gen_palindrome(half: usize, is_odd: bool) -> usize{
    let mut result = half;
    // 奇数桁の場合(n+1)/2が上半分となる
    // そのため下半分を考える際に真ん中の桁を再度足さないようにする
    let mut tmp = if is_odd {half/10}else{half};

    while tmp > 0{
        result = result*10 + tmp%10;
        tmp = tmp/10
    }
    result
}

fn main(){
    input!{
        a: usize,
        n: usize,
    }

    let mut ans:usize = 0;
    let max_digits = n.to_string().len();

    for i in 1..=max_digits {
        // 格桁で半分より上の桁の数の範囲を求める
        // 3桁の場合、上2つの桁(10 ~ 99)
        let half_len = (i+1)/2;
        let start = 10_usize.pow((half_len - 1) as u32);
        let end = 10_usize.pow(half_len as u32) -1;

        let mut is_odd = true;
        if i%2 == 0{is_odd = false}

        for j in start..=end{
            let palindrome:usize = gen_palindrome(j, is_odd);
            if n < palindrome{
                break;
            }
            // println!("palindrome: {}", palindrome);

            if n_number_palindrome(a, palindrome){
                // println!("{}", i);
                ans += palindrome;
            }
        }
    }

    println!("{}", ans);
}


