#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n;
    cin >> n;

    vector<int> coordinate(n);
    vector<vector<int>> pos_pop(n, vector<int> (2));

    rep(i, n){
        cin >> coordinate[i];
    }
    rep(i, n){
        pos_pop[i][0] = coordinate[i];
        cin >> pos_pop[i][1];
    }

    sort(pos_pop.begin(),pos_pop.end(),[](const vector<int> &alpha,const vector<int> &beta){return alpha[0] < beta[0];});
    sort(coordinate.begin(),coordinate.end());


    vector<ll> vec_acu(n);
    ll sum2 = 0;
    rep(i, n){
        sum2 += pos_pop[i][1];
        vec_acu[i] = sum2;
    }

    int m;
    cin >> m;
    rep(i,m){
        int l, r;
        cin >> l >> r;
        auto ite_l = lower_bound(coordinate.begin(), coordinate.end(), l);
        auto ite_r = upper_bound(coordinate.begin(), coordinate.end(), r);

        // cout << "L " << *ite_l;
        // cout << " R : " << *(ite_r-1)<< endl;

        int index_l =  (int)(ite_l - coordinate.begin())-1;
        int index_r =  (int)(ite_r - coordinate.begin())-1;


        if(index_l == n-1 || index_r < 0){
            cout << 0 << endl;
        }else{
            if(index_r == n) index_r = n-1;
            if(index_l < 0){
                cout << vec_acu[index_r]<< endl;
            }else{
                cout << vec_acu[index_r] - vec_acu[index_l]<< endl;
            }
        }
    }



}