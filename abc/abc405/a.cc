#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int r, x;
    cin >> r >> x;

    if (x == 1 && 1600 <=r && r <= 2999){
        cout << "Yes" << endl;
    }else if (x == 2 && 1200 <=r && r <= 2399){
        cout << "Yes" << endl;
    }else{
        cout << "No" << endl;
    }
}