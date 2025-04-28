#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n, m, q;
    cin >> n >> m >> q;

    unordered_map<int, unordered_set<int>> permision;

    rep(i, q){
        int k;
        cin >> k;
        if(k == 1){
            int x, y;
            cin >> x >> y;
            permision[x].insert(y);
        }else if(k == 2){
            int x;
            cin >> x;
            permision[x].insert(0);
        }else{
            int x, y;
            cin >> x >> y;
            if (permision[x].count(0) || permision[x].count(y)){
                cout << "Yes" << endl;
            }else{
                cout << "No" << endl;
            }
        }
    }

  
}