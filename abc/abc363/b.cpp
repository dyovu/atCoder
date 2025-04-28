#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int n, t, p, cnt = 0;
  cin >> n >> t >> p;
  vector<int> length(n);


  rep(i, n){
    int k;
    cin >> k;
    if(k > t){
      cnt ++;
    }else{
      length[i] = k;
    }
  }
  
  sort(length.rbegin(), length.rend());
  if(cnt >= p){
    cout << 0 << endl;
  }else{
    cout << t -length[p-cnt-1] << endl;
  }
}