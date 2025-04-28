#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  string s, t;
  cin >> s >> t;

  int n = (int)s.length();

  int cnt = 0;
  vector<string> moji(n);

  rep(i, n){
    if(s[i] > t[i]){
      s[i] = t[i];
      moji[cnt] = s;
      cnt ++;
    }
  }

  rep(i, n){
    int k = n-1-i;
    if(s[k] < t[k]){
      s[k] = t[k];
      moji[cnt] = s;
      cnt ++;
    }
  }

  cout << cnt << endl;
  rep(i, cnt){
    cout << moji[i] << endl;
  }

}
