#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

/* 
  "https://drken1215.hatenablog.com/entry/2023/10/21/021300"
  「平均値の最大化」-> 「二分探索」の問題
  まず、答えとなる塩分濃度の平均値を仮定する。k個選択した時その仮定との誤差によって二分探索していく。
  追加するk個の食塩量、食塩水の量を分母分子で足し合わせたものが、ある濃度x以上になるという不等式を変形する。
  濃度が高くても送料が少なければ全体に与える影響が少ない！！！！！！！
*/ 

int main(){
  int n, k;
  cin >> n >> k;

  vector<double> w(n), p(n);
  for (int i = 0; i < n; ++i) cin >> w[i] >> p[i];

  // rep(i, n){
  //   cout << mass[i][0] << " " << mass[i][1] << endl;
  // }

  double ng = 0.0, ok = 100;
  rep(i, 50){
    double mid = (ng + ok)/2;
    vector<double> tmp(n);
    for (int j = 0; j < n; ++j){
      tmp[j] = (p[j] - mid) * w[j];
    }
    sort(tmp.rbegin(), tmp.rend());
    // sort(tmp.begin(), tmp.end(), greater<double>());
    double sum = reduce(tmp.begin(), tmp.begin()+k, 0.0);
    if(sum >= 0) ng = mid;
    else ok = mid;
  }

  cout << fixed << setprecision(9) << ok << endl;
  // cout << ok << endl;

}