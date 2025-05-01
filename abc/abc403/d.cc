#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    // 入力される数列の各項は10^6以下の整数である
    // つまり、mapに入力された数とその回数を格納していくと、mapのサイズとmapで保存している数の最大値が異なってしまう。
    // そこで、mapの代わりにvectorを使うことにする。
    // 今回の制約として各項は10^6以下であるから、vectorのサイズを10^6+5程度にしておけば、vectorの各要素に入力された数を格納していくことができる。
    // また10^6+5は計算量的にfor分で1回回すことができるから、今回の問題においてはvectorを使う

    int n, d;
    cin >> n >>  d;

    int M = 1000005;
    vector<int> sequence(M);
    rep (i, n){
        int a;
        cin >> a;
        sequence[a]++;
    }

    if (d == 0){
        int ans = 0;
        rep(i, M){
            if (sequence[i] > 0) ans += sequence[i] - 1;
        }
        cout << ans << endl;
        return 0;
    }

    int dp_sum = 0;
    int ans = 0;
    rep(i, d){
        vector<int> v;
        for(int j = i; j < M; j += d){
            v.push_back(sequence[j]); // ベクターに対するpuhs_backの計算量は考慮しなくていいの？
        }

        int size = v.size();
        vector dp(size+1, vector<int>(2, 0));
        
        for (int j = 0; j < size; j++){
            dp[j+1][0] = min(dp[j][0], dp[j][1]) + v[j];
            dp[j+1][1] = dp[j][0];
        }
        dp_sum += min(dp[size][0], dp[size][1]);
    }

    cout << dp_sum << endl;
}