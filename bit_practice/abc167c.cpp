#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  int n, m, x, min_cost=-1;
  cin >> n >> m >> x;

  vector<vector<int>> compre(n,vector<int>(m));
  vector<int> price(n);

  rep(i, n){
    cin >> price[i];
    rep(j, m){
      cin >> compre[i][j];
    }
  }

  for(int i=0; i < (1<<n); i++){
    bool flag = true;
    int tmp_cost = 0;
    vector<int> tmp_compre(m);
    for(int j=0; j<n; j++){
      if((i>>j)&1){
        tmp_cost += price[j];
        rep(k, m){
          tmp_compre[k] += compre[j][k];
        }
      }
    }
    rep(j, m){
      if(tmp_compre[j] < x){
        flag = false;
      }
    }
    if(flag && (min_cost==-1 || min_cost>tmp_cost)){
      min_cost = tmp_cost;
    }
  }

  cout << min_cost << endl;

}