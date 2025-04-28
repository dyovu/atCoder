#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  int n, m, l;
  cin >> n >> m;
  vector<int> num(n);


  rep(i, n){
    int k;
    cin >> k;
    num[i] = k; 
  }

  rep(i, n){
    m -= num[i];
    if(m < 0){
      cout << i << endl;
      break;
    }
  }

  if(m >= 0){
    cout << n << endl;
  }


}