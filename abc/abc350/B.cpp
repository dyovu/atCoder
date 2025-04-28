#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  int n, q, pull_num=0;
  cin >> n >> q;
  vector<int> count(n);


  rep(i, q){
    int k;
    cin >> k;
    count[k-1]++;
  }

  // cout << "count : " <<count[0] << endl;

  rep(i, n){
    pull_num += count[i]%2;
  }

  cout << n-pull_num << endl;

  
}