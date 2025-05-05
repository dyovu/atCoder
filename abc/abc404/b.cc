#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n;
    cin >> n;

    vector<vector<int>> s(n, vector<int>(n));
    vector<vector<int>> t(n, vector<int>(n));


    rep(i, n){
        rep(j, n){
            char c;
            cin >> c;
            if (c == '.') {
                s[i][j] = 0;
            } else {
                s[i][j] = 1;
            }
        }
    }

    rep(i, n){
        rep(j, n){
            char c;
            cin >> c;
            if (c == '.') {
                t[i][j] = 0;
            } else {
                t[i][j] = 1;
            }
        }
    }

    // sをtに近づける

    vector<int> sum_aaa (4, 0);

    rep(i, 4){
        int cnt = 0;
        rep(i, n){
            rep(j, n){
                cnt += abs(s[i][j] - t[i][j]);
            }
        }

        sum_aaa[i] = cnt+ i;

        vector<vector<int>> tmp(n, vector<int>(n));
        rep(i, n){
            rep(j, n){
                tmp[j][n - 1 - i] = s[i][j];
            }
        }
        s = tmp;
    }

    rep(i, 4){
        // cout << sum_aaa[i] << endl;
    }

    cout << *min_element(sum_aaa.begin(), sum_aaa.end()) << endl;

  
}