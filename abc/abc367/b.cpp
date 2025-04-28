#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  string num;
  int last = 0;
  cin >> num;
  last = num.length();


  reverse(num.begin(), num.end());
  // cout << num << endl;

  rep(i, 4){
    // cout << "k : " << k << endl;
    // cout << "num[k] : " << num[i] << endl;
    if(num[i] == '0' || num[i] == '.'){
      last --;
    }else{
      break;
    }
  }

  reverse(num.begin(), num.end());

  cout << num.substr(0, last) << endl;
  

}