#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


int main(){
    string s;
    cin >> s;
    ll ans = 0;

    rep(i, s.size()) {
        ans++;
        if (i == 0){
            continue;
        }
        if((s[i] - '0') - (s[i-1] - '0') <= 0){
            ans += (s[i-1] - '0') - (s[i] - '0');
        } else {
            ans += 10 - (s[i] - '0') + (s[i-1] - '0');
        }
        // cout << ans << endl;
    }
    ans += s[(int)s.size() - 1] - '0';
    // cout<< s[(ll)s.size() - 1] << endl;
    cout << ans << endl;
    
}