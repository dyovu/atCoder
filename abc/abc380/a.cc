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
    int a=0, b=0, c=0;


    for(auto v: n){
        if(v == '1') a +=1;
        if(v == '2') b +=1;
        if(v == '3') c +=1;
    }
    
    // cout << a << b << c << endl;

    if(a == 1 && b ==2 && c ==3) cout << "Yes" << endl;
    else cout << "No" << endl;
  
}