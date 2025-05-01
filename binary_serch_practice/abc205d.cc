#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n, q;
  cin >> n >> q;

  vector<ll> seq(n);
  vector<ll> candi_num(n);
  rep(i, n) {
    ll k;
    cin >> k;
    seq[i] = k;
    candi_num[i] = k - i-1;
  }

  rep(i, q){
    // cout << "---------" << endl;
    ll k; 
    cin >> k;

    auto ite = lower_bound(candi_num.begin(), candi_num.end(), k);
    int index = ite - candi_num.begin();

    // cout << index << endl;

    if(index == 0){
      cout << k << endl;
    }else{
      cout << seq[index-1] + k - candi_num[index-1] << endl;
    }
  }
  
}