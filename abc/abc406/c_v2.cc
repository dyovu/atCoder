#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;


int main(){
    int n;
    cin >> n;

    // prevより大きければ1, 
    vector<int> up_down(n, 0);

    int first;
    cin >> first;
    int prev = first;
    up_down[0] = 2;

    rep(i, n-1) {
        int now;
        cin >> now;
        if (now > prev){
            up_down[i+1] = 1;
        }
        prev = now;
    }

    int up = 0;
    int pre_up = 0;
    int state = 2;
    ll ans =0;

    rep(i, n){
        if (up_down[i] == 1){
            if (state == 0){
                up = 0;   
            }
            state = 1;
            up += 1;
        }else if (up_down[i] == 0){
            if (state == 1){
                ans += up*pre_up;
                pre_up = up;
            }
            state = 0;
        }

        // cout << "i: " << i << ", up: " << up << ", pre_up: " << pre_up << ", state: " << state << endl;
    }

    if (state == 1){
        ans += up*pre_up;
    }

    cout << ans << endl;

}


