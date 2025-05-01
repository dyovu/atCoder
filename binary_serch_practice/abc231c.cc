#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG


int main(){
  int n, q;
  cin >> n >> q;

  vector<int> stature(n);
  rep(i, n){
    cin >> stature[i];
  }
  
  sort(stature.begin(), stature.end());

  /*
    lower booundをつかう
  */
  auto f = [&](int x) -> int {
    auto ite = lower_bound(stature.begin(),stature.end(), x);
    return stature.end() - ite;
  };

  rep(i, q){
    int x, ng = 0, ok = n;
    cin >> x;

    while(abs(ok-ng) >1){
      int mid = (ng + ok) >> 1;
      if(stature[mid] < x){
        ng = mid;
      }else{
        ok = mid;
      }
    }
    cout << n - ok << endl;

    // cout << f(x) << endl;

  }

}