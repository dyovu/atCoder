use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }

    let root = a[0];
    // 先頭で待ちがいない数字を入れてく
    // rootとなる数字のフィルタリング、すでに出たものはダメ
    let mut appeared: HashSet<usize> = HashSet::new();

    // HashMapで現在の先頭と長さを持つ？
    let mut sequences: HashMap<usize, usize>= HashMap::new();
    for key in a.iter(){
        let mut found = false;
        appeared.insert(*key);

        // ここで要素のミュータブルな借用が行われ、if letのブロック終了時に解放されます
        if let Some(v) = sequences.get_mut(&(key - 1)) {
            *v += 1;
            found = true;
        }
        if found {
            sequences.remove(&key);
            continue
        }

        
    }

}
