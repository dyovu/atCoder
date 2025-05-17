#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;






int main(){
    int a, b, c, d;
    cin >> a >> b >> c >> d;

    if (a <  c){
        cout << "No" << endl;
        return 0;
    }else if(a == c && b < d){
        cout << "No" << endl;
        return 0;
    }

    cout << "Yes" << endl;

}


