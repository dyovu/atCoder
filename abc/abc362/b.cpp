#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG

int main(){
  vector<int> a(2);
  vector<int> b(2);
  vector<int> c(2);
  vector<int> side(3);

  cin >> a[0] >> a[1] >> b[0] >> b[1] >> c[0] >> c[1];

  int A, B, C;
  A = pow((b[0] - c[0]), 2) + pow((b[1] - c[1]), 2);
  B = pow((a[0] - c[0]), 2) + pow((a[1] - c[1]), 2);
  C = pow((b[0] - a[0]), 2) + pow((b[1] - a[1]), 2);

  side[0] = A;
  side[1] = B;
  side[2] = C;

  sort(side.begin(), side.end());

  if(side[2] == side[0] + side[1]){
    cout << "Yes" << endl;
  }else{
    cout << "No" << endl;
  }

}