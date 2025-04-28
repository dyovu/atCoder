#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG


int main(){
  int n, l, r;

  cin >> n >> l >> r;

  for(int i=1; i<l ;i++){
    cout << i << ' ';
  }

  for(int i=0; i<r-l+1 ;i++){
    cout << r-i << ' ';
  }

  for(int i=r; i<n ;i++){
    cout << i+1 << ' ';
  }

  cout << endl;

}
