#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  int n;
  int64_t sum_shoulder=0, max_height=0;
  cin >> n;
  vector<vector<int>> height_combi(n, vector<int> (2));

  rep(i, n){
    // cout << i << endl;
    int j, k;
    cin >> j >> k;
    height_combi.at(i).at(0) = j;
    height_combi.at(i).at(1) = k;
    sum_shoulder += j;
  }

  //  cout << sum_shoulder << endl;

  rep(i ,n){
    int64_t temp_height = sum_shoulder - height_combi.at(i).at(0) + height_combi.at(i).at(1);

    if(temp_height > max_height){
      max_height = temp_height;
    }
  }

  cout << max_height << endl;


}