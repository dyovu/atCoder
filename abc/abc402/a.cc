
#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    string m, ans;
    cin >> m;

    rep(i, m.length()){
        if(isupper(m[i])){
            ans += m[i];
        }
    }
    cout << ans << endl;
}