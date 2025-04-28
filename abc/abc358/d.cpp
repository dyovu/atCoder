#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG

int main(){
  int n, m;
  ll cost=0;
  cin >> n >> m;

  priority_queue<int, vector<int>, greater<int>> quantity;
  priority_queue<int, vector<int>, greater<int>> min_quantity;

  rep(i, n){
    int k;
    cin >> k;
    quantity.push(k);
  }

  rep(i, m){
    int k;
    cin >> k;
    min_quantity.push(k);
  }

  while(1){
    int tmp_quantity = quantity.top();
    int tmp_min_quantity = min_quantity.top();

    if(tmp_quantity >= tmp_min_quantity){
      cost += tmp_quantity;
      min_quantity.pop();
    }
    quantity.pop();

    if(min_quantity.size() == 0){
      cout << cost << endl;
      break;
    }

    if(quantity.size() == 0){
      cout << "-1" << endl;
      break;
    }
  }

}