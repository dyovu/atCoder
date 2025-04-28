#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  string s, t;
  int l=0;

  cin >> s >> t;
  queue<int> num;

  for(int i=0; i < t.size(); i++){
    if(t.at(i) == s[l]){
      num.push(i+1);
      l++;
    }
  }
  // cout << "size:" << num.size() << endl;;

  int m = num.size();
  
  rep(i, m){
    cout << num.front() << ' ';
    num.pop();
  }

  

  // for(int i=0; i<m; i ++){
  //   cout << num.front() << ' ';
  //   num.pop();
  // }

  cout << endl;


}