#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  string s;
  cin >> s;
  string flag = "Yes";

  map<char, int> count_char;
  map<char, int> count_num;

  for(char c: s){
    if(count_char.count(c)){
      count_char[c]++;
    }else{
      count_char[c] = 1;
    }
  }

  for(auto v : count_char){
    auto value = v.second;
    if(count_num.count(value)){
      count_num[value]++;
    }else{
      count_num[value] = 1;
    }
  }

  for(auto v : count_num){
    auto value = v.second;
    if(value != 2){
      flag = "No";
    }
  }

  cout << flag << endl;
  

}