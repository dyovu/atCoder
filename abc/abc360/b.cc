#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


int main(){
  string s, t;
  cin >> s >> t;

  int t_len = (int) t.length();
  int s_len = (int) s.length();

  for(int w = 1; w < s_len; w++){
    rep(c, w){
      string tmp = "";
      for(int i = c; i < s_len; i += w){
        tmp += s[i];
      }
      if(tmp == t){
        cout << "Yes" << endl;
        return 0;
      }
    }
  }
  cout << "No" << endl;
}