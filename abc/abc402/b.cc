#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n;
    cin >> n;

    queue<int> wait_list;

    rep(i, n){
        int a;
        cin >> a;
        if (a == 1){
            int b;
            cin >> b;
            wait_list.push(b);
        }else{
            int c = wait_list.front();
            wait_list.pop();
            cout << c << endl;
        }
    }
  
}