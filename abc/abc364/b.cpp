#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int main(){
  int h, w, sx, sy;
  string x;
  cin >> h >> w >> sx >> sy;
  vector<string> grid (h);
  sx --;
  sy --;

  rep(i, h){
    string k;
    cin >> k;
    // cout << typeid(k).name() << endl;
    grid[i] = k;
  }

  cin >> x;

  for(auto v: x){
    // cout << v << endl;
    switch(v){
      case 'U':
        if(sx != 0 && grid[sx-1][sy] == '.'){
          sx --;
        }
        break;
      case 'R':
        if(sy != w-1 && grid[sx][sy+1] == '.'){
          sy ++;
        }
        break;
      case 'D':
        if(sx != h-1 && grid[sx+1][sy] == '.'){
          sx ++;
        }
        break;
      case 'L':
        if(sy != 0 && grid[sx][sy-1] == '.'){
          sy --;
        }
        break;
    }
  }

  cout << sx+1 << ' '<<  sy+1 << endl;

}