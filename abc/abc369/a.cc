#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int ma, mb;
  cin >> ma >> mb;

  int a = max(ma, mb);
  int b = min(ma, mb);

  int dif = a- b;

  // cout << dif << endl;

  if(dif == 0) cout << 1 << endl;
  else if(dif%2 == 0) cout << 3 << endl;
  else cout << 2 << endl;

}