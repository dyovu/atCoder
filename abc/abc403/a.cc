#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n;
    cin >> n;

    int sum=0;

    rep(i, n){
        int k;
        cin >>k;
        if (i%2==0){
            sum+= k;
        }
    }
    cout << sum << endl;
}