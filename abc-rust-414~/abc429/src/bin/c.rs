use proconio::input;
// use std::io::{self, BufRead};
use std::{collections::*, hash::Hash};

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn combination_memo(n: usize, r: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if r > n { return 0; }
    if r == 0 || r == n { return 1; }
    
    if let Some(&result) = memo.get(&(n, r)) {
        return result;
    }
    
    let result = combination_memo(n-1, r-1, memo) + combination_memo(n-1, r, memo);
    memo.insert((n, r), result);
    result
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut value_index:HashMap<usize, Vec<usize>> = HashMap::new();
    let mut count:HashMap<usize, usize> = HashMap::new();

    for (index, value) in a.iter().enumerate(){
        // value_index.entry(*value).or_insert(Vec::new()).push(index);
        *count.entry(*value).or_insert(0) += 1;
    }

    // println!("{:?}", count);
    let mut ans:usize = 0;

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    for (i, cnt) in count{
        let comb = combination_memo(cnt, 2, &mut memo);
        ans += (n-cnt)*comb;
    }

    println!("{}", ans);

}