#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n;
    cin >> n;
    ll sum = 0;

    vector<int> vec_a(n);

    rep (i, n){
        int a;
        cin >> a;
        sum += a;
        vec_a[i] = a;
    }

    ll ans = 0;
    rep(i, n-1){
        ans += vec_a[i] * (sum - vec_a[i]);
        sum -= vec_a[i];
    }
    
    cout << ans << endl;
}