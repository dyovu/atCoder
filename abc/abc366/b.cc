#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n, size = 0, index = 0;
  cin >> n;
  vector<string> s(n);

  rep(i, n){
    string k;
    cin >> k;
    
    if(size < (int) k.length()){
      size = (int) k.length();
      index = i;
    }
    s[i] = k;
  }

  vector<string> ans(size);
  rep(i, size){
    rep(j, n){
      int k = n - j - 1;
      if((int)s[k].length() < i +1){
        ans[i] += "*";
      }else{
        ans[i] += s[k][i];
      }
    }
  }

  rep(i, size){
    // cout << ans[i].back() << endl;
    while (ans[i].back() == '*') {
      // cout << "here " << i << endl; 
      ans[i].pop_back();
    }
    cout << ans[i] << endl;
  }

}