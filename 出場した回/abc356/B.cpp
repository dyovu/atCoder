#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG


int main(){
  int n, m;
  cin >> n >> m;
  string ans = "Yes";

  vector<int> border(m);

  vector<vector<int>> nutrition(n, vector<int>(m));
  vector<int> sum(m);

  rep(i, m){
    cin >> border[i];
  }

  rep(i, n){
    rep(j, m){
      int k;
      cin >> k;
      nutrition.at(i).at(j) = k;
      sum[j] += k;
    }
  }

  rep(i, m){
    if(sum[i] < border[i]){
      ans = "No";
    }
  }

  cout << ans << endl;

  
}