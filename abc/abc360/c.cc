#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


int main(){
  int n;
  cin >> n;

  vector<vector<int>> box_weight(n, vector<int> (2));
  vector<int> box(n);
  vector<int> weight(n);
    


  rep(i, n) cin >> box[i];
  rep(i, n) cin >> weight[i];

  int count = 0;
  rep(i, n){
    int b = box[i], w = weight[i];
    box_weight[b-1][0] += w;
    box_weight[b-1][1] = max(box_weight[b-1][1], w);
  }

  for(auto v: box_weight){
    count += v[0] - v[1];
  }

  cout << count << endl;
  
}