#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n;
  cin >> n;

  int m= 0;
  vector<int> set = {1, 1, 3};
  ll t = 0;
  
  vector<ll> vec(n);

  rep(i, n){
    // cout <<"--------" << endl;
    ll k;
    cin >> k;

    vec[i] = k;
    
    int remainder = k%5;
    ll quotient = (ll)k/5;

    t += quotient*3;

    while(remainder > 0){
      remainder -= set[m];
      m++;
      if(m == 3) m = 0;
      t ++;
    }
    // cout << t << endl;
  }

  cout << t << endl;

}