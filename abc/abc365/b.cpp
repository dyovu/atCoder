#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int n, l=0, m=0;
  cin >> n;

  map<int, int> dic;
  rep(i, n){
    int k;
    cin >> k;

    int s = min(l, k);
    l = max(l, k);
    m = max(m, s);

    dic[k] = i+1;
  }
  cout << dic[m] << endl;
}