#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int q;
  cin >> q;

  map<int, int> query;
  queue<int> ans;
  // cout << query.size() << endl;

  rep(i, q){
    int no, num;
    cin >> no;

    if(no == 1){
      cin >> num;
      if(query.count(num)){
        query[num] += 1;
      }else{
        query[num]  =1;
      }
    }else if(no == 2){
      cin >> num;
      query[num] -= 1;
      if(query[num] == 0){
        query.erase(num);
      }
    }else if(no == 3){
      ans.push(query.size());
    }
  }

  while (!ans.empty()) {
    cout << ans.front() << endl;  // 先頭の値を出力
    ans.pop();  // 先頭の値を削除
  }



}