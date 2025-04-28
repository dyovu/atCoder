#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int n;
  cin >> n;
  vector<string> taste(n);
  bool t1 = false, t2 = false;

  rep(i, n){
    string k;
    cin >> k;
    if(t1 && k == "sweet" && i != n-1){
      t2 = true;
    }else if(!(t1) && k == "sweet"){
      t1 = true;
    }else{
      t1 = false;
    }
  }


  if(t2){
    cout << "No" << endl;
  }else{
    cout << "Yes" << endl;
  }

}