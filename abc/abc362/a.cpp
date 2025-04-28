#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG

int main(){
  int r, g, b;
  cin >> r >> g >> b;
  string c;
  cin >> c;

  if(c == "Red"){
    cout << min(g,b) << endl;
  }else if(c == "Blue"){
    cout << min(g,r) << endl;
  }else if(c == "Green"){
    cout << min(b,r) << endl;
  }

}