#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main(){
    string t, u;
    cin >> t >> u;

    if (t.length() < u.length()){
        cout << "No" << endl;
        return 0;
    }

    rep (ite, t.length()){
        if (ite + u.length() > t.length()) {
            break;
        }
        int count = 0;
        if (t[ite]== '?' || t[ite] == u[0]){
            count++;
            for (int i=1; i < int(u.length());  i++){
                if (t[ite+i]== '?' || t[ite+i] == u[i]){
                    count++;
                }else{
                    break;
                }
            }
        }
        if (count == u.length()){
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;
    return 0;
  
}