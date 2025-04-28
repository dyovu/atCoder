#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG

/*
  じゃんけんの手i番目までの最適な出し方 → i = n として勝利数の最大値を求める
  各手の出し方、R, P, S毎のi回目の勝利の最大値をだす

  多分DP使う目印として、i番目までの勝利数の最大値が i-1番目までの手とi番目の手から求められるから？
*/

int main() {
    using namespace std;
    unsigned N;
    cin >> N;
    string S;
    cin >> S;

    array<unsigned, 3> dp{};
    auto&& [rock, scissors, paper]{dp};

    for(const auto c : S){
        // 直前に出していなかった手を出すことができる
        // n回目にR,S,Pをそれぞれ出すためにはn-1回目にそれ以外の手を出す必要がある、
        // そのためn回目にR,S,Pを出す際のそれまでの最大値は下のようになる
        // n回目に出す手３つ全てのパターンを記録しながらいく、n+1回目に必要になってくるから
        dp = {max(scissors, paper), max(rock, paper), max(rock, scissors)};
        // 負ける手を出すことはできない = 勝ち数の最大値を 0 にする
        // 勝つ手を出したら最大値 +1
        if (c == 'R') {
            scissors = 0;
            ++paper;
        } else if (c == 'S') {
            paper = 0;
            ++rock;
        } else if (c == 'P') {
            rock = 0;
            ++scissors;
        }
    }

    cout << max(max(dp[0],dp[1]), dp[2]) << endl;

    return 0;
}
