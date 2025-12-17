// use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let t:usize = lines.next().unwrap().unwrap().parse().unwrap();

    

    for _ in 0..t{
        let n:usize = lines.next().unwrap().unwrap().parse().unwrap();
        let s = lines.next().unwrap().unwrap();
        let s = format!("0{}", s);
        let s_bytes = s.as_bytes();
        // println!("{}", s);
        // println!("{:?}", s_bytes);

        // 要素が全て0で2^n個のvector
        /*
         * インデックスを状態としてそこに到達できるかどうかを記入していく
         * 例えばn=3だったら2^3 = 8この要素を持つ配列になる
         */ 
        let mut reachable = vec![false; 1<<n];
        // 最初は何も混ざっていない状態0とする
        reachable[0] = true;

        // println!("{:?}", reachable);


        /*
         * ここで言う`状態`って言うのは基本的に10進数で表されるが
         * ビットシフトをすることで、どの薬品が混ざっているかを確認できる
         * 例:) n=4で状態7の時, 7=0111 (これは薬品1, 2, 3が混ざった状態)
         * これをn=0..4 でビットシフトすると、それぞれの薬品が混ざっているかがわかる
         */

        /*
         * 状態0からn-1までを順に検証していく
         * 
         * 自分的にはまず薬品を1つだけのパターンを全て検証してその次に、1つ追加して2つのパターンを検証....
         * と考えていた。
         * 
         * でも、状態を昇順に検証していくき、そこから薬品を1つ追加してできる状態が到達可能かをメモしていけばいい
         * この順番での操作によって、処理中の状態が到達可能かどうかはこれ以前に決定している
         */
        for i in 0..(1 << n){
            // この状態に到達可能でなければ次に進む
            // 最初は状態0であり、何も混ざっていない状態
            if !reachable[i]{
                continue;
            }

            // 次にこの状態からそれぞれの薬品を追加できるかを試みる
            // すでに入っている場合はパス
            // 薬品を追加した状態が`危険`か`安全`かをreachableにメモしていく

            for j in 0..n{
                if (i >> j) & 1 == 0{ // 薬品jがまだ入っていない場合 (iのj桁目のフラグが立っているかどうかを調べ得ている)
                    // まだ入っていない薬品を追加する
                    // 薬品jが入っていないのでbitシフトで、jのフラグを立てる。
                    // それをiとor演算することで薬品jを追加した新しい状態を定義できる
                    // 
                    // 考え的には2進数のどこが1かを考えて、1の部分が入っている薬品となる。
                    // 出力される値は10真数になるので、その値をindexとするsの
                    let next = i | (1<<j);
                    if s_bytes[next] == b'0'{
                        reachable[next] = true;
                    }
                }
            }
        }

        if reachable[(1<<n) -1]{
            println!("Yes");
        }else{
            println!("No");
        }

    }


}