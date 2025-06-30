#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;



int main(){
    int n;
    cin >> n;

    map<int, vector<int>> mp;
    int prev = 0;
    int pre_prev = 0;

    rep(i, n) {
        int now;
        cin >> now;
        vector<int> tmp(3, 0);

        if(i == 0){
            mp[i] = tmp;
            prev = now;
            continue;
        }

        if(prev < now){
            tmp[0] = 1;
        }
        mp[i] = tmp;

        if (i < 2) {
            pre_prev = prev;
            prev = now;
            continue;
        }

        if (pre_prev < prev && prev > now){
            mp[i-1][1] = 1;
        }else if(pre_prev > prev && prev < now){
            mp[i-1][2] = 1;
        }

        pre_prev = prev;
        prev = now;
    }


    // // mapの中身を表示
    // for (const auto& pair : mp) {
    //     cout << "Key: " << pair.first << ", Value: ";
    //     for (int i = 0; i < 3; i++) {
    //         cout << pair.second[i] << " ";
    //     }
    //     cout << endl;
    // }

    int prefix = 0;
    int suffix = 0;
    
    // 条件3か4を満たしてる状況をすうじでひょうげん、現在の最新をセットする
    int previous = 0;
    int pre_previous = 0;
    int pre_previous_index = 0;
    int previous_index = 0;
    vector<vector<int>> ans_vec(n, vector<int>(2, 0));

    rep (i ,n){
        if (mp[i][0] == 1){
            prefix += 1;
        }else {
            prefix = 0;
        }

        if (pre_previous * previous != 0){
            suffix++;
        }
        
        if (mp[i][1] == 1 ){
            if(pre_previous * previous != 0 && pre_previous != previous){
                ans_vec[pre_previous_index][1] = suffix;
            }
            ans_vec[i][0] = prefix;
            pre_previous_index = previous_index;
            previous_index = i;
            prefix = 0;
            suffix = 0;
            pre_previous = previous;
            previous = 3;
        }else if (mp[i][2] == 1){
            if(pre_previous * previous != 0 && pre_previous != previous){
                ans_vec[pre_previous_index][1] = suffix;
            }
            ans_vec[i][0] = prefix;
            pre_previous_index = previous_index;
            previous_index = i;
            prefix = 0;
            suffix = 0;
            pre_previous = previous;
            previous = 4;
        }else{
            ans_vec[i][0] = 0;
        }
    }

    // 最後のsuffixをセット
    if(pre_previous * previous != 0 && pre_previous != previous){
        ans_vec[pre_previous_index][1] = suffix;
    }
    

    // ans_vecの中身を表示
    for (int i = 0; i < n; i++) {
        cout << "ans_vec[" << i << "]: ";
        for (int j = 0; j < 2; j++) {
            cout << ans_vec[i][j] << " ";
        }
        cout << endl;
    }


    // ans_vecの中身を計算
    ll ans = 0;
    for (int i = 0; i < n; i++) {
        if (ans_vec[i][0] == 0 && ans_vec[i][1] == 0) {
            continue;
        }
        ans += ans_vec[i][0] * ans_vec[i][1];
    }

    cout << ans << endl;


}


