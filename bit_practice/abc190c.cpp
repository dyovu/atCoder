#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  int n, m, k, count=0;
  cin >> n >> m;

  vector<vector<int>> rulus(m, vector<int>(2));

  rep(i, m){
    cin >> rulus[i][0] >> rulus[i][1];
  }


  cin >> k;
  vector<vector<int>> put_off(k, vector<int>(2));

  rep(i, k){
    cin >> put_off[i][0] >> put_off[i][1];
  }


  for(int i=0; i<(1<<k); i++){
    unordered_set<int> plate_on_ball{};
    int tmp_count=0;
    rep(j ,k){
      // cout << " : " << ((i >> j) & 1) << endl;
      plate_on_ball.insert(put_off[j][(i >> j) & 1]);
      // cout << put_off[j][(i >> j) & 1] << endl;
    }
    for(int j=0; j<m; j++){
      if(plate_on_ball.count(rulus[j][0]) && plate_on_ball.count(rulus[j][1])){
        tmp_count++;
      }
      // cout << "j: " << j << "  tmp_count: " <<tmp_count << endl;
    }
    count = max(tmp_count, count);
  }

  

  cout << count << endl;

}