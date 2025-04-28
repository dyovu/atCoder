#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n , count_zero = 0, cnt = 0;
  cin >> n;

  bool flag = true;

  vector<int> vec(n);
  rep(i, n) cin >> vec[i];

  auto check = [&] (vector<int> & x) -> bool{
    if(x[0] == 0) count_zero ++;
    if(x[1] == 0) count_zero ++;
    if(count_zero >= n-1) return false;
    else return true;
  };

  while(flag){
    cnt ++;
    sort(vec.rbegin(), vec.rend());
    vec[0] -= 1;
    vec[1] -= 1;
    flag = check(vec);
  }

  cout << cnt << endl;
}