#include <bits/stdc++.h>
using namespace std;
using ll = long long;
#define rep(i, n) for(ll i = 0; i < (ll)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
// cout << typeid(aaaa).name() << endl;

int main(){
  ll n, m, p, total_amount = 0;
  cin >> n >> m >> p;

  vector<ll> main_dish(n);
  vector<ll> side_dish(m);
  vector<ll> cum_sum(n);

  rep(i, n) cin >> main_dish[i];
  sort(main_dish.begin(), main_dish.end());

  rep(i, n){
    if(i == 0) cum_sum[i] = main_dish[i];
    else cum_sum[i] = cum_sum[i-1] + main_dish[i];
  }
  
  rep(i, m) cin >> side_dish[i];
  sort(side_dish.begin(), side_dish.end());
  
  
  rep(i, m){
    if(side_dish[i] >= p){
      // cout << "side_dish is exceed 'p' " << endl;
      total_amount += p*n*(m-i);
      break;
    }else{
      auto ite = lower_bound(main_dish.begin(), main_dish.end(), p-side_dish[i]);
      ll index = (ite - main_dish.begin());
      // cout << "----------" << endl;    
      // cout << index << endl;
      if(index == n){
        // cout << "sum is always not exceed 'p' " << endl;
        //maindishがいくらでもpを超えない場合
        total_amount += cum_sum[n-1] + n*side_dish[i];
      }else if(index == 0){
        // cout << "sum is always exceed 'p' " << endl;
        //maindishがいくらでもpを超える場合 
        total_amount += n*p;
      }else{
        total_amount += cum_sum[index-1] + index*side_dish[i];
        total_amount += (n - index)*p;
      }
    }
  }

  cout << total_amount << endl;

}