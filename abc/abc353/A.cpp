#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
  int a, ans;

  ans = -1;
  cin>>a;


  vector<int> height(a);
  
  rep(i, a){
    cin >> height[i];
    if (height[0]<height[i]){
      ans = i+1 ;
      break;
    }
  }

  cout << ans << endl;

}