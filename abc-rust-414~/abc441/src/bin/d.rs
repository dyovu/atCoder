use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        l: usize,
        s: usize,
        t: usize,
        a: [[usize; 3]; m],
    }

    let mut map:HashMap<usize, Vec<Vec<usize>>> = HashMap::new();

    for i in a.iter(){
        map.entry(i[0]).or_insert(Vec::new()).push(vec![i[1], i[2]]);
    }

    // println!("{:?}", map);

    let mut deque:VecDeque<Vec<usize>> = VecDeque::new();
    let start = vec![1, 0, 0];
    deque.push_back(start);

    let mut ans:BTreeSet<usize> = BTreeSet::new();

    while 0 < deque.len(){
        let state = deque.pop_front().unwrap();


        let next_vertex = match map.get(&state[0]) {
            Some(t) => t,
            None => { continue },
        };

        let count = state[1] + 1;

        for next in next_vertex{
            let cost = state[2] + next[1];
            if count == l{
                if s <= cost && cost <= t{
                    ans.insert(next[0]);
                }
            }else if count < l{
                deque.push_back(vec![next[0], count, cost])
            }
        }

        // println!("deque: {:?}", deque);
        // println!("ans: {:?}", ans);
    }

    // println!("{:?}", ans);

    for i in ans{
        print!("{} ", i)
    }


}