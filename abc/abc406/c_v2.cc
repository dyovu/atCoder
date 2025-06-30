#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


int main(){
    int n;
    cin >> n;

    vector<int> a(n, 0);

    rep(i, n) {
        cin >> a[i];
    }


    // prevより大きければ1, 
    vector<int> up_down(n, 0);
    up_down[0] = 2;
    rep(i, n-1) {
        up_down[i+1] = (a[i] < a[i+1]) ? 1 : 0;
    }

    // for (int i = 0; i < n; i++) {
    //     cout << up_down[i] << " ";
    // }cout << endl;


    int up = 0;
    int pre_up = 0;
    int state = 2;
    ll ans =0;
    rep(i, n){
        if (up_down[i] == 1){
            state = 1;
            up += 1;
            // cout << "i: " << i << ", up: " << up << ", pre_up: " << pre_up << ", state: " << state << endl;
        }else if (up_down[i] == 0){
            if (state == 1){
                ans += (ll)up*pre_up;
                pre_up = up;
                up = 0;
                // cout << "ans : " << ans << endl;
            }
            state = 0;
            // cout << "i: " << i << ", up: " << up << ", pre_up: " << pre_up << ", state: " << state << endl;
        }
        // cout << "i: " << i << ", up: " << up << ", pre_up: " << pre_up << ", state: " << state << endl;
    }

    if (state == 1){
        ans += (ll)up*pre_up;
    }

    cout << ans << endl;
}


