#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

// 方針
// i = 1 -> n　において、i回目が偶数である時と奇数である時をそれぞれ求めいていく。

int main(){
  int n;
  cin >> n;

  vector<ll> exp(n);
  rep(i ,n) cin >> exp[i];

  vector<ll> even(n);
  vector<ll> odd(n);

  even[0] = 0;
  odd[0] = exp[0];

  for(int i=1; i< n; i++){
    even[i] = max((even[i-1]), (odd[i-1] + 2*exp[i]));
    odd[i] = max((odd[i-1]), (even[i-1] + exp[i]));
  }

  cout << max(even[n-1], odd[n-1]) << endl;
}