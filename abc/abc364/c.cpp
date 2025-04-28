#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int n;
  ll x, y, swt = 0, spc=0;
  cin >> n >> x >> y;

  int f = n;
  vector<int> sweet(n);
  vector<int> spicy(n);

  rep(i, n){
    cin >> sweet[i];
  }

  rep(i, n){
    cin >> spicy[i];
  }

  sort(sweet.rbegin(), sweet.rend());
  sort(spicy.rbegin(), spicy.rend());

  rep(i, n){
    swt += sweet[i];
    spc += spicy[i];
    if(swt > x || spc > y){
      f = i+1;
      break;
    }
  }

  cout << f << endl;

}