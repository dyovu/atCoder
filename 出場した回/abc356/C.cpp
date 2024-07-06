#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG


int main(){
  int n, m, k, ans=0;
  cin >> n >> m >> k;

  vector<int> num(m);
  vector<char> judg(m);
  vector<vector<int>> bit(m, vector<int>(n));

  rep(i, m){
    cin >> num[i];
    rep(j, num[i]){
      int k;
      cin >> k;
      bit[i][k-1] = 1;
    }
    cin >> judg[i];
  }
  //ここまで入力受け取り


  for(int i=0; i<(1<<n); i++){
    vector<int> key_num(m);
    bool flag = true;

    rep(j, n){
      if((i>>j)&1){
        //ここまでふつうのbit全探索
        rep(k, m){
          key_num[k] =  key_num[k] + bit[k][j];
          // cout << "key_num[k]: " << (key_num[k]) << endl;
          // cout << "(bit[k][j]): " << (bit[k][j]) << endl;
        }
      }
    }

    rep(j,m){
      if(key_num[j] >= k && judg[j] == 'o' ){
        continue;
      }else if(key_num[j] < k && judg[j] == 'x' ){
        continue;
      }else{
        flag = false;
        break;
      }
    }

    if(flag){
      ans ++;
    }

  }

  cout << ans << endl;


}