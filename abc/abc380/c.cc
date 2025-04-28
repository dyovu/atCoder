#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    int n, k;
    cin >> n >> k;
    string s;
    cin >> s;
    

    vector<string> one, zero;
    int index_one = 0;
    string current = "";
    char prev = s[0];
    bool one_is_first = prev == '1';
    

    for(char c : s){
        if(c == prev){
            current += c; 
        }else{
            if (prev == '1'){
                if(index_one == k-1){
                    one[index_one-1] = one[index_one-1]+current;
                }else{
                    one.push_back(current);
                }       
                index_one++;
            }
            else {
                if(index_one == k){
                    zero[k-1] = zero[k-1]+current;
                }else zero.push_back(current);
            }
            current = c;
        }
        prev = c;
    }

    if (prev == '1'){
        if(index_one == k-1){
            one[index_one-1] = one[index_one-1]+current;
        }else{
            one.push_back(current);
        }
    }
    else{
        if(index_one == k){
            zero[k-1] = zero[k-1]+current;
        }else zero.push_back(current);
    }


    string ans = "";
    int m = min(one.size(),zero.size());
    rep(i, m){
        if(one_is_first){
            ans += one[i];
            ans += zero[i];
        }else{
            ans += zero[i];
            ans += one[i];
        }
    }
    if(one.size() < zero.size()) ans += zero[zero.size()-1];
    if(one.size() > zero.size()) ans += one[one.size() -1];

    cout << ans << endl;
  
}