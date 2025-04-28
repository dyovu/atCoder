#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int n, q;
  cin >> n >> q;
  vector<int> aSeq(n);
  vector<int> bSeq(n);
  vector<int> kSeq(n);
  
  rep(i, n){
    cin >> aSeq[i];
  }

  sort(aSeq.begin(), aSeq.end());


  rep(i, q){
    
    int b, k;
    cin >> b >> k;

    auto f = [&](int x) -> bool {
      // (# of points in [b-x, b+x]) >= k
      // xはbからどれくらい離れているか
      auto lb = lower_bound(aSeq.begin(), aSeq.end(), b - x);
      auto ub = upper_bound(aSeq.begin(), aSeq.end(), b + x);
      // ub - lbで b-x <= n <= b+x (bからx離れている要素の個数) が求められる
      return ub - lb >= k;
    };

    int ng =-10 , ok = (int)2e8;
    while(ok -ng > 1){
      int mid = (ok+ng) >> 1;

      if(f(mid)){
        ok = mid;
      }else{
        ng =mid;
      }
    }
    cout<< ok << endl;

  }


  

  

}