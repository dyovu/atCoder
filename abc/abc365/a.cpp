#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int y;
  cin >> y;

  if(y%400 == 0 || (y%4 == 0 && y%100 != 0)){
    cout << 366 << endl;
  }else{
    cout << 365 << endl;
  }

}