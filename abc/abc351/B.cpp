#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main() {
  int n ;
  cin >> n;
  vector<string> a_vec(n);
  vector<string> b_vec(n);

  rep(j ,n){
    cin >> a_vec[j];
  }
  rep(j ,n){
    cin >> b_vec[j];
  }


  rep (i, n){
    rep(j ,n){
      if(a_vec[i][j] != b_vec[i][j]){
        cout << i+1 <<' ' <<  j+1 << endl;
      }
    }
  }


}