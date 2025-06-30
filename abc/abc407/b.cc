#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;

int main(){
    int x, y;
    cin >> x >> y;
    double sum = 36;
    double oppo = 0;

    rep(i, 6){
        int one = i + 1;
        rep(j, 6){   
            int two = j + 1; 
            if((one+two < x)  && (abs(one-two) < y)){
                oppo ++;
                // cout << i << " " << j << endl;
            }
        }
    }

    // cout <<oppo << endl;

    cout << fixed << setprecision(10) << ((double)36-oppo) / sum << endl;


}