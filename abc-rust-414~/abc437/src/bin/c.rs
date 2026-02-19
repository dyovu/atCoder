use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut reindeer: Vec<(i64, i64)> = Vec::with_capacity(n);
        let mut total_power: i64 = 0;

        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let w: i64 = iter.next().unwrap().parse().unwrap();
            let p: i64 = iter.next().unwrap().parse().unwrap();
            reindeer.push((w, p));
            total_power += p;
        }

        // Wi + Pi の昇順でソート
        reindeer.sort_by_key(|&(w, p)| w + p);

        let mut ans = 0;
        let mut sum_wp: i64 = 0;

        for (w, p) in &reindeer {
            sum_wp += w + p;
            if sum_wp <= total_power {
                ans += 1;
            } else {
                break;
            }
        }

        println!("{}", ans);
    }
}