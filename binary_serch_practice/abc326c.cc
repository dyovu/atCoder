#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


void printVector(vector<int> &v){
  for(auto i: v){
    cout << i << ' ';
  }
  cout << "/n" << endl;
}

int main(){
  
  int n, m, count = 0;
  cin >> n >> m;

  vector<int> seq(n);
  rep(i, n)  cin >> seq[i];
  sort(seq.begin(), seq.end());
  // printVector(seq);
  
  

  rep(i, n){
    // cout << "--------" << endl;;
    // cout << "i: " << i << endl;;
    int x = seq[i];
    double xm = x + m -0.5;
    

    auto Iter1 = lower_bound(seq.begin(),seq.end(), x);
    auto Iter2 = upper_bound(seq.begin(),seq.end(), xm);

    count = max(count, (int)(Iter2 - Iter1));
    // cout << Iter2 - Iter1 << endl;
  }

  cout << count << endl;



}