use proconio::input;
use std::cmp::max;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    // 入力をまとめて読み込む
    input! {
        n: usize,
        a: [[usize; 3]; n],
    }

    // dp[i] = 体の重さが i のときの最大幸福度
    // 初期値 -1 は「到達不可能」を表す
    let mut dp: Vec<i64> = vec![-1];
    dp[0] = 0; // 重さ0, 幸福度0 からスタート

    for i in 0..n {
        let w: usize = a[i][0];
        let h: i64 = a[i][1] as i64;
        let b: i64 = a[i][2] as i64;

        // 前の状態を保持 (cloneして使う)
        let prev = dp.clone();
        
        // 配列サイズを拡張 (新しい重さ w の分だけ増える可能性がある)
        // 拡張部分は -1 で埋める
        dp.resize(prev.len() + w, -1);

        // prev の全要素を走査
        for (i, &val) in prev.iter().enumerate() {
            if val == -1 {
                continue; // 到達不可能な状態からは遷移しない
            }

            // 1. 頭に取り付ける (体の重さは i のまま)
            // 現在の dp[i] と、新しい候補 (val + h) を比較
            dp[i] = max(dp[i], val + h);

            // 2. 体に取り付ける (体の重さは i + w になる)
            // 現在の dp[i+w] と、新しい候補 (val + b) を比較
            dp[i + w] = max(dp[i + w], val + b);
        }
    }

    // 答えを求める
    // 条件: 頭の重さ <= 体の重さ
    // 全重量 = 頭 + 体 なので、条件は「体の重さ >= 全重量 / 2」と同義
    // dpのサイズが全重量+1 なので、そのまま半分以降を見れば良い
    
    let total_possible_weight = dp.len();
    // 半分(切り上げ)以上のインデックスを対象にする
    let threshold = total_possible_weight / 2;

    let ans = dp.iter()
        .enumerate()
        .filter(|&(weight, _)| weight >= threshold) // 条件を満たす重さ
        .map(|(_, &val)| val) // 値を取り出す
        .max() // その中での最大値
        .unwrap_or(0); // 何もなければ0 (制約上ありえないが念のため)

    println!("{}", ans);
}