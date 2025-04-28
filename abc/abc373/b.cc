#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    string m = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    map<char, int> s;
    rep(i, 26){
        char k;
        cin >> k;
        s[k] = i+1;
    }

    int prev = s['A'], cnt = 0;
    rep(i, 25){
        cnt += abs(prev - s[m[i+1]]);
        prev = s[m[i+1]];
        // cout << cnt << endl;
    }

    cout << cnt << endl;

    
}