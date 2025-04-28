#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n;
  cin >> n;
  vector<vector<int>> seq(n, vector<int>(n));

  rep(i, n){
    rep(j, i+1){
      cin >> seq[i][j];
      // cout << seq[i][j];
    }
    // cout << endl;
  }

  int comp = 1;
  rep(i, n){
    if(i > comp-1){
      comp = seq[i][comp-1];
    }else{
      comp = seq[comp-1][i];
    }
    // cout << "i : " <<  i << " comp : " << comp << endl;
  }

  cout << comp << endl;
  // cout << seq[n-1][comp-1] << endl;

}
