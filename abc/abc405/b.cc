#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n, m;
    cin >> n >> m;

    deque<int> deq_a;

    set<int> set_a;
    int count = 0;

    rep(i, n){
        int a;
        cin >> a;

        if (a <= m && !set_a.count(a)){
            set_a.insert(a);
            if (set_a.size() == m){
                cout << n - count << endl;
                return 0;
            }
        }
        count ++;
    }    
    cout << 0 << endl;
}