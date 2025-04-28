#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  int n, m;

  cin >> n >> m;

	bitset<60> bn(n);
  bitset<60> bm(m);
  bitset<60> npm(bn & bm);
	cout << bn << endl;

  cout << npm << endl;


  cout << typeid(npm).name() << endl;


}