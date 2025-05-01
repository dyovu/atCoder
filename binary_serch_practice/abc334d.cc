#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


void printVector(vector<int> &v){
  for(auto i: v){
    cout << i << ' ';
  }
  cout << '/n' << endl;
}

int main (){
  int n, q;
  cin >> n >> q;
  vector<ll> reinder_num(n);
  vector<ll> sums (n+1);

  rep(i, n) cin >> reinder_num[i];

  sort(reinder_num.begin(), reinder_num.end());
  partial_sum(reinder_num.begin(), reinder_num.end(), sums.begin()+1);

  rep(i, q){
    ll x;
    cin >> x;
    int ok = n+1, ng = 0;

    // cout << upper_bound(sums.begin(), sums.end(), x) - sums.begin() -1 << endl;

    while(abs(ok -ng) >1){
      int mid = (ok + ng) >> 1;
      
      if(sums[mid] > x) ok = mid;
      else ng = mid;
    }

    cout << ng << endl;
  }

}