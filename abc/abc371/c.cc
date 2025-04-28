#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


#include <iostream>
#include <set>
#include <utility>
#include <vector>
#include <numeric>
#include <algorithm>

// グラフ理論？における同型とは、頂点と辺のつながりだけをみて同じ形であるもの
// 必ずしも２つのグラフで頂点の番号が一致していなくてもよい

// この問題ではn=8なのでグラフHの各点に対応するグラフGの点を全探索で求めて良い

int main(){
    int n, mg;
    cin >> n >> mg;

    // 逆向きの変の組み合わせも入れると便利
    // set<vector<int>> G{vector<int> (2)};
    set<pair<int, int>> G;
    rep(i, mg){
        int a, b;
        cin >> a >> b;
        // G.insert({a-1, b-1});
        // G.insert({b-1, a-1});
        G.emplace(a-1, b-1);
        G.emplace(b-1, a-1);
    }
    // sort(G.begin(),G.end(),[](const vector<int> &alpha,const vector<int> &beta){return alpha[0] < beta[0];});

    int mh; cin >> mh;
    // set<vector<int>> H{vector<int> (2)};
    set<pair<int, int>> H;
    rep(i, mh){
        int a, b;
        cin >> a >> b;
        // H.insert({a-1, b-1});
        // H.insert({b-1, a-1});
        H.emplace(a-1, b-1);
        H.emplace(b-1, a-1);
    }
    // sort(H.begin(),H.end(),[](const vector<int> &alpha,const vector<int> &beta){return alpha[0] < beta[0];});

    
    // コストを入れる配列
    vector a(n, vector<int>(n));
    rep(i,n){
        for(int j=i+1; j < n; j++){
            cin >> a[i][j];
            a[j][i] = a[i][j];
        }
    }

    // 頂点の並べ替えをする配列
    vector<int> p(n);
    iota(begin(p), end(p), 0);

    // 最大値を考えて自分で設定する。
    // int ans = (int)4*7*1e6;
    int ans{28000000};
    do {
        int sum = 0;
        for(int i=0; i<n; i++){
            for(int j=0; j<i; j++){
                // なぜかGとHが逆だと答えが違う
                sum += a[i][j] * (H.count({i,j}) != G.count({p[i], p[j]}));
            }
        }
        ans = min(ans, sum);
    } while (next_permutation(p.begin(), p.end()));

    cout << ans << endl;

}