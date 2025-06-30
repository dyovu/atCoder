#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;

int main(){
    int a, b;
    cin >> a >> b;

    double fl = (float)a/b;
    // cout << fixed << setprecision(6) << fl << endl;
    int i = (int)round(fl);
    cout << i << endl;
    // if (i-fl >= 0.5) {
    //     cout << i + 1 << endl;
    // } else {
    //     cout << i << endl;
    // }
    return 0;
    
}