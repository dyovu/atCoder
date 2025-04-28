#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n, m;
    cin >> n >> m;

    map<int, int> x;
    rep(i, m){
        int h;
        char s;
        cin >> h >> s;

        if(!(x.count(h)) && s == 'M'){
            x[h] = 1;
            cout << "Yes" << endl;
        }else{
            cout << "No" << endl;
            continue;
        }
    }
  
}