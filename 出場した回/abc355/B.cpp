#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main() {
  int n, m, p=0;
  cin >> n >> m;
  bool a1 =false;

  int l =n+m;

  unordered_set<int> n_num(n);
  unordered_set<int> m_num(m);
  priority_queue<int> sum;

  
  for(int i=0; i<n; i++){
    int k ;
    cin >> k;
    n_num.insert(k);
    sum.push(k);
  }
  for(int i=0; i<m; i++){
    int k ;
    cin >> k;
    m_num.insert(k);
    sum.push(k);
  }

  for(int i=0; i<n+m; i++){
    if(n_num.count(sum.top())){
      p ++;
      sum.pop();
    }else{
      p=0;
      sum.pop();
    }
    if(p==2){
      a1=true;
    }
  }

  if(a1){
    cout << "Yes" << endl;
  }else{
    cout << "No" << endl;
  }



}