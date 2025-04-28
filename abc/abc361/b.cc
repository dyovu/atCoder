#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  vector<int> cube1(6);
  vector<int> cube2(6);

  rep(i, 6) cin >> cube1[i];
  rep(i, 6) cin >> cube2[i];

  // 1次元の判定を繰り返す.
  bool bx = cube2[3] <= cube1[0] ||  cube1[3] <= cube2[0];
  bool by = cube2[4] <= cube1[1] ||  cube1[4] <= cube2[1];
  bool bz = cube2[5] <= cube1[2] ||  cube1[5] <= cube2[2];

  if((!bx && !by) && !bz){
    cout << "Yes" << endl;
  }else{
    cout << "No" << endl;
  }

}