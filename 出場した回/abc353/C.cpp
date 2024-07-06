#include <bits/stdc++.h>
using namespace std;
using ll = long long;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
  int n;
  long long ans = 0, cnt = 0;;
  cin >> n;

  vector<int> num(n);

  rep(i, n){
    cin >> num[i]; 
    ans += num[i]*(n-1);
  }

  sort(num.begin(), num.end());

  int k = n;

  rep(i, n){
    k = max(k, i+1);
    while(k-1 > i && num[k-1] + num[i] >= 100000000){
      k--;
    }
    cnt += n-k;
  }

  ans -= cnt*100000000;

  cout << ans << endl; 

}
