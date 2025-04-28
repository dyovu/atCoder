#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    string n;
    cin >> n;
    int sum = 0; 

    queue<int> a;

    for(auto v: n){
        if(v == '|'){
            if(sum > 0){
                a.push(sum);
                sum = 0;
            }
        }else{
            sum += 1;
            // cout << "sum : " <<sum << endl;;
        }
        
        
    }
    // cout << a.size() << endl;

    while (!a.empty()) {
        cout << a.front()<< ' ' << endl;  // 先頭の値を出力
        a.pop();  // 先頭の値を削除
    }
  
}