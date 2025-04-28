#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    ll  n, k;
    cin >> n >> k;
    const ll MOD = 1000000000;

    if (n < k){
        cout << 1 << endl;
        return 0;
    }else if (n == k){
        cout << k % MOD << endl;
        return 0;
    }

    
    deque<ll> dq(k, 1);
    ll current = k % MOD;

    for (ll i = 0; i < n-k; i++){
        if (i == n) {
            cout << current % MOD << endl;
            return 0;
        }
        ll front = dq.front();
        dq.pop_front();

        dq.push_back(current);
        current = (current - front + MOD) % MOD; 
        current = (current + dq.back()) % MOD;
        // cout << "front: " << front << " current: " << current << endl;
    }

    cout << current << endl;

}