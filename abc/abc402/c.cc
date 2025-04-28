#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n, m;
    cin >> n >> m;

    unordered_map<int, unordered_set<int>> cusine;
    unordered_map<int, vector<int>> ingredient;

    rep(i, m){
        int a;
        cin >> a;
        unordered_set<int> s;
        rep(j, a){
            int b;
            cin >> b;
            ingredient[b].push_back(i);
            s.insert(b);
        }
        cusine[i] = s; 
    }


    int count = 0;
    rep(i,n){
        int d;
        cin >> d;
        vector<int> elm_cusine = ingredient[d];

        for (int c: elm_cusine){
            cusine[c].erase(d);
            if (cusine[c].empty()){
                count ++;
            }
        }
        cout << count << endl;
    }
  
}