use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let q: usize = first_line.parse().unwrap();

    let mut blocks: Vec<(u64, u64)> = Vec::new(); // (count, value)
    let mut block_index: usize = 0;
    let mut offset: u64 = 0; // 現在のブロック内で消費済みの個数

    for line in lines.take(q){
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let query_type: u64 = iter.next().unwrap().parse().unwrap();

        if query_type == 1{
            let c: u64 = iter.next().unwrap().parse().unwrap();
            let x: u64 = iter.next().unwrap().parse().unwrap();
            blocks.push((c, x));
        }else{
            let mut k: u64 = iter.next().unwrap().parse().unwrap();
            let mut sum: u64 = 0;

            while k > 0 {
                let (count, value) = blocks[block_index];
                let remaining = count - offset;

                if k >= remaining {
                    sum += remaining * value;
                    k -= remaining;
                    block_index += 1;
                    offset = 0;
                } else {
                    sum += k * value;
                    offset += k;
                    k = 0;
                }
            }
            println!("{}", sum);
        }
    }
}