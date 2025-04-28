#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n, k;
  ll sum = 0;
  cin >> n >> k;

  vector<int> seq(n);
  rep(i, n){
    int k;
    cin >> k;
    seq[i] = k;
    sum += k;
  }

  sort(seq.begin(), seq.end());

  int minb = 0, maxb = seq[n-1];
  rep(i, k+1){
    if(seq[n+i-k-1] - seq[i] < maxb - minb){
      maxb = seq[n+i-k-1];
      minb = seq[i];
    }
  }

  cout << maxb - minb << endl;

  // int lo = 0, hi = n-1;
  // rep(i, k){
  //   ll ave = sum/(n-i);
    
  //   if(abs(ave-(ll)seq[lo]) < abs(ave-(ll)seq[hi])){
  //     sum -= (ll)seq[hi];
  //     hi --;
  //   }else{
  //     sum -= (ll)seq[lo];
  //     lo ++;
  //   }
  // }
  // cout << seq[hi] - seq[lo] << endl;

}