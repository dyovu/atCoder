#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;
// ---------------------------------------------------
// 現在残っている壁の位置をset を用いて管理する
// 縦横それぞれの行・列に残っている壁の位置をset内に保持しておく
// setに対して2分探索を用いることで高速に値を求めることができる
// ある値より小さい値ははlower_boundの値を一つ戻すことで得られる
// ---------------------------------------------------


int main(){
  int h, w, q;
  cin >> h >> w >> q;

  vector<set<int>> rows(h);
  vector<set<int>> columns(w);

  rep(i, h){
    rep(j, w){
      rows[i].insert(j);
      columns[j].insert(i);
    }
  }

  rep(i, q){
    int r, c;
    cin >> r >> c;
    r --, c --;

    if(rows[r].count(c)){
      rows[r].erase(c);
      columns[c].erase(r);
    }else{
      {
        auto ite = columns[c].lower_bound(r);
        if(ite != begin(columns[c])){
          int x = (int)*prev(ite);
          rows[x].erase(c);
          columns[c].erase(x);
        }
      }
      {
        auto ite = columns[c].lower_bound(r);
        if(ite != end(columns[c])){
          int x = (int)*ite;
          rows[x].erase(c);
          columns[c].erase(x);
        }
      }
      {
        auto ite = rows[r].lower_bound(c);
        if(ite != begin(rows[r])){
          int x = (int)*prev(ite);
          rows[r].erase(x);
          columns[x].erase(r);
        }
      }
      {
        auto ite = rows[r].lower_bound(c);
        if(ite != end(rows[r])){
          int x = (int)*ite;
          rows[r].erase(x);
          columns[x].erase(r);
        }
      }
      
    }
  }

  ll cnt = 0;
  rep(i, h){
    cnt += (ll)rows[i].size();
    // cout << cnt << endl;
  }
  cout << cnt << endl;
}


// void upp(int k, int l, vector<set<int>> &r, vector<set<int>> &c);
// void down(int k, int l, vector<set<int>> &r, vector<set<int>> &c);
// void left(int k, int l, vector<set<int>> &r, vector<set<int>> &c);
// void right(int k, int l, vector<set<int>> &r, vector<set<int>> &c);


// void upp(int k, int l, vector<set<int>> &r, vector<set<int>> &c){
//   auto ite = c[l].lower_bound(k);
//   if(ite != begin(c[l])){
//     int x = (int)*prev(ite);
//     r[x].erase(l);
//     c[l].erase(x);
//   }
//   return ;
// }

// void down(int k, int l, vector<set<int>> &r, vector<set<int>> &c){
//   auto ite = c[l].lower_bound(k);
//   if(ite != end(c[l])){
//     int x = (int)*ite;
//     r[x].erase(l);
//     c[l].erase(x);
//   }
//   return ;
// }

// void left(int k, int l, vector<set<int>> &r, vector<set<int>> &c){
//   auto ite = r[k].lower_bound(l);
//   if(ite != begin(r[k])){
//     int x = (int)*prev(ite);
//     r[k].erase(x);
//     c[x].erase(k);
//   }
//   return ;
// }

// void right(int k, int l, vector<set<int>> &r, vector<set<int>> &c){
//   auto ite = r[k].lower_bound(l);
//   if(ite != end(r[k])){
//     int x = (int)*ite;
//     r[k].erase(x);
//     c[x].erase(k);
//   }
//   return ;
// }