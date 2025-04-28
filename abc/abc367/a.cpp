#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int a, b, c;
  cin >> a >> b >> c;
  bool flag =true;
  

  do{
    if(b==a) flag = false;
    if(b==23) b=0;
    else b++;
  }while(b != c);

  if(c==a) flag = false;


  if(flag) cout << "Yes" << endl;
  else cout << "No" << endl;
  

}