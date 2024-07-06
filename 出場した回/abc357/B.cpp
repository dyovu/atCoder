#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  int count_u=0, count_l=0;
  string phrase, cha;
  cin >> phrase;

  for(auto v: phrase){
    if(isupper(v)){
      count_u ++;
    }else{
      count_l ++;
    }
  }

  if(count_l > count_u){
    for(auto v: phrase){
      if(isupper(v)){
        cha += v+32;
      }else{
        cha += v;
      }
    }
  }else{
    for(auto v: phrase){
      if(isupper(v)){
        cha += v;
      }else{
        cha += v-32;
      }
    }
  }

  cout << cha << endl;
  
}