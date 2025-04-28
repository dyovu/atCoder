#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n;
    cin >> n;
    int a = 0, b = 0;
    vector<int> veca(n);
    vector<int> vecb(n);
    rep(i, n){
        int k;
        cin >> k;
        veca[i] = k;
        a = max(a, k);
    }
    sort(veca.rbegin(), veca.rend());
    rep(i, n){
        int k;
        cin >> k;
        vecb[i] = k;
        b = max(b, k);
    }
    sort(vecb.rbegin(), vecb.rend());

    cout << veca[0] + vecb[0] << endl;
    // cout << a+b << endl;
  
}