#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG




bool check(const string &s, int k) {
    int n = s.size();
    for (int i = 0; i <= n - k; ++i) {
        string substr = s.substr(i, k);
        if (substr == string(substr.rbegin(), substr.rend())) {
            return true;
        }
    }
    return false;
}


int main(){
  int n, k;
  ll cnt=0;
  string s;
  cin >> n >> k >> s;

  sort(s.begin(), s.end());

  // vector<char> letter(n);
  // rep(i, n){
  //   cin >> letter[i];
  // }

  set<string> a;

  do{
    a.insert(s);
  }while(next_permutation(s.begin(), s.end()));

  for (const auto &perm : a) {
    if (!check(perm, k)) {
      cnt++;
    }
  }

  
  cout << cnt << endl;

  
}