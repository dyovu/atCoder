use std::io::{self, BufRead};


/*
* 配列全体を毎回コピーして新しく配列を作るのはTLEになるからleading_indexで論理的な先頭を保持する
*/
fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut num_seq:Vec<usize> = vec![0; n];
    for i in 0..n{
        num_seq[i] = i+1;
    }
    let mut leading_index:usize = 0;

    for line in lines.take(q) {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let query_type: usize = iter.next().unwrap().parse().unwrap();
        
        if query_type == 1{
            let p: usize = iter.next().unwrap().parse().unwrap();
            let x: usize = iter.next().unwrap().parse().unwrap();
            num_seq[(leading_index + p - 1) % n] = x;
        }else if query_type == 2{
            let p: usize = iter.next().unwrap().parse().unwrap();
            println!("{}", num_seq[(leading_index + p - 1) % n]);
        }else if query_type == 3{
            let k: usize = iter.next().unwrap().parse().unwrap();
            leading_index = (leading_index + k) % n;
        }
    }
}