#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n;
    cin >> n;

    int count = 0;
    bool is_login = false;

    for (int i=0; i < n; i++) {
        string s;
        cin >> s;

        

        if (s == "login") {
            is_login = true;
        }else if (s == "logout") {
            is_login = false;
        }

        if ((s == "private") && (!is_login)){
            count++;
        }

        // cout << "is_login : " << is_login << endl;
        // cout << "count : " << count << endl;

    }

    // cout << "^---------" << endl;

    cout << count << endl;


}