#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n, k;
  cin >> n >> k;

  vector<int> vec(n);
  rep(i, n) cin >> vec[i];

  rep(i, n){
    int l = (i + n-k )%n;
    cout << vec[l] << " ";
  }
  cout << endl;


}