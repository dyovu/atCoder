#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n, d;
    cin >> n, d;

    map<int, int> sequence;

    rep (i, n){
        int a;
        cin >> a;
        if (sequence.find(a) != sequence.end()){
            sequence[a] += 1;
        }else{
            sequence[a] = 1;
        }
    }
    for (auto it = sequence.begin(); it != sequence.end(); it++){
        cout << it->first << " " << it->second << endl;
    }

    



}