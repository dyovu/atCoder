#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG

int main(){
  int n, a, sum = 0;
  cin >> n >> a;

  vector<int> time(n);

  rep(i ,n){
    int k;
    cin >> k;

    if(sum>k){
      sum = sum + a;
      cout << sum << endl;
    }else{
      sum = k + a;
      cout << sum << endl;
    }
  }
  
}