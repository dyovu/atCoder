#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    string n;
    cin >> n;

    set<int> chars;

    rep(i, 26){
        chars.insert(i);
    }


    for (auto c : n){
        int k = int(c);
        chars.erase(k-97);
        // cout << c << " " << k << endl;
    }

    cout << char(*begin(chars)+97) << endl;  
}