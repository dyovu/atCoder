#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
  int h, n=0, l=0;
  cin>>h;

  while(h >= l){
    l += pow(2, n);
    n++;
  }
  n = int(n);

  cout << n << endl;

}