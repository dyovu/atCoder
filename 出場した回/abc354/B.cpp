#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main() {
  int n, m, l=0, sum=0;
  cin>>n;

  map<string, int> dic{};

  for(int i=0; i<n; i++){
    string name;
    int rate;
    cin >> name >> rate;

    dic[name] = rate;
    sum += rate;
  
  }

  m = sum%n;

  for (auto const& [k, v] : dic){
    if(l == m){
      cout << k<< endl;
      break;
    }
    l++;
  }


}