#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG

int main(){
  int n, m;
  cin >> n >> m;
  int min_store = n;

  vector<string> taste(n);
  

  rep(i ,n){
    cin >> taste[i];
    // cout << "taste" << i << " = " << taste[i] << endl;;
  }

  for(int i=0; i<(1<<n); i++){
    unordered_set<int> sold_taste(m);
    int count=0;
    for(int j=0; j<n; j++){
      if((i>>j)&1){
        count ++;
        for(int k=0; k<m; k++){
          if(taste[j][k]=='o'){
            sold_taste.insert(k);
          }
        }
      }
      // cout << "count" << i << ' ' << j << " = " << count << endl;
    }

    // cout << "sold_taste.size() : " << sold_taste.size() <<endl;

    if(sold_taste.size()==m && min_store > count){
      // cout << "called patch min" << endl;
      min_store = count;
    }
  }

  cout << min_store << endl;
  
}