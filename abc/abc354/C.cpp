#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main() {
  int n, l=0;
  cin>>n;

  vector<vector<int>> dic(n);
  vector<int> card(0);

  for(int i=0; i<n; i++){
    int power;
    int cost;
    cin >> power >> cost;

    dic[i].push_back(power);
    dic[i].push_back(cost);
    dic[i].push_back(i);
  
  }

  sort(dic.rbegin(), dic.rend());
  card.push_back(dic[0][2]+1);


  for (int i=0; i<n-1; i++){
    // cout << i << ":" << dic[i][0]<<endl;

    if(dic[l][1] > dic[i+1][1]){
      l = i+1;
      card.push_back(dic[i+1][2]+1);
    }
  }

  sort(card.begin(), card.end());

  cout << card.size() << endl;

  for(int i=0; i<card.size()-1; i++){
    cout << card[i] << " ";
  }
  cout << card.back() << endl;


}