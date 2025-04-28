#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    char a, b, c;
    cin >> a >> b >> c;

    if(a == '>' && b == '>'){
        if(c == '>') cout << 'B' << endl;
        else cout << 'C' << endl;
    }else if(a == '<' && c == '>'){
        if(b == '>') cout << 'A' << endl;
        else cout << 'C' << endl;
    }else if(b == '<' && c == '<'){
        if(a == '>') cout << 'A' << endl;
        else cout << 'B' << endl;
    }
  
}