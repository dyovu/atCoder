#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int cnt = 0;
    rep(i, 12){
        string k;
        cin >> k;
        if(k.length() == i+1){
            cnt ++;
        }
    }
    cout << cnt << endl;
  
}