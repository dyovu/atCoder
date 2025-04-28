#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int n;
  ll m;
  cin >> n >> m;

  vector<int> te(n);
  // for(auto &v : te) cin >> v;

  rep(i, n){
    int k;
    cin >> k;
    te[i] = k;
  }
  // cout << "------------" <<endl;

  sort(te.begin(),te.end());

  ll sum = reduce(te.begin(), te.end(), 0ll);

  int ok = 0, ng = 1000000000;
  while(abs(ok - ng) > 1){
    int mid = (ok + ng) >> 1;
    ll s = 0;

    for(auto &v :te){
      s += min(mid, v);
    }
    if(s > m){
      ng = mid;
    }else{
      ok = mid;
    }
  }

  // cout << "ng" << ng << endl;
  // cout << "ok" << ok << endl;

  if(sum <= m){
    cout << "infinite" << endl;
  }else{
    cout << ok << endl;
  }


}