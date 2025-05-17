#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;

int main(){
    int n, k;
    cin >> n >> k;

    ll ans = 1;

    rep(i, n){
        ll a;
        cin >> a;
        
        string current = to_string(ans);
        string multiplier = to_string(a);
        
        // llがオーバーフロー?
        if (current.length() + multiplier.length() - 1 > k) {
            ans = 1;
        } else {
            ans *= a;
            if (to_string(ans).length() > k) {
                ans = 1;
            }
        }
    }

    cout << ans << endl;
}