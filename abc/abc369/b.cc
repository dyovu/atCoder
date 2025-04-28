#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
  int n, count = 0;
  cin >> n;

  map <char, int> psosition;

  rep(i, n){
    char hand;
    int key;

    cin >> key >> hand;

    if(!psosition.count(hand)){
      psosition[hand] = key;
    }else{
      count += abs(psosition[hand] - key);
      psosition[hand] = key;
    }
  }

  cout << count << endl;
}