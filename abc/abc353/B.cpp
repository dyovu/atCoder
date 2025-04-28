#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
  int n, k, k__, ans;
  cin >> n >> k;

  k__ = k;
  ans = 0;
  vector<int> nop(n);

  rep(i, n){
    cin >> nop[i];
    if(k__ >= nop[i]){
      k__ -= nop[i];
    }else{
      k__ = k-nop[i];
      ans ++;
    }
  }

  cout << ans << endl;

}
