#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int l, r;
  cin >> l >> r;
  if(l == r) cout << "Invalid" << endl;
  else if(l == 1) cout << "Yes" << endl;
  else if(1 == r) cout << "No" << endl;
}
