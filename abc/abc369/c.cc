#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n, dif = 0, prev = 0;
  ll count = 0;
  cin >> n;
  // vector<int> seq(n);

  int subarr_count = 0, reset_num = 0;

  rep(i, n){
    int preseent;
    cin >> preseent;

    if( (i == 0 || i == 1) || abs(preseent - prev) == dif){
      dif = abs(preseent - prev);
      prev = preseent;
      subarr_count += 1;
    }else{
      count += (ll)subarr_count*(subarr_count + 1)/2;
      dif = abs(preseent - prev);
      prev = preseent;
      subarr_count = 2;
      reset_num += 1;
    }
  }
  count += (ll)subarr_count*(subarr_count + 1)/2 - reset_num;

  cout << count << endl;
  
}