#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

void draw(int n){
  // int k = (int)pow(3, n);

  if(n == 1){
    rep(i, 3){
      rep(j, 3){
        if(j == 1 && i==1){
          cout << '.';
        }else{
          cout << '#';
        }
        if(j==2){
          cout << endl;
        }
      }
    }
  }else{
    draw(n-1);
  }
  

}

int main(){
  int n;
  cin >> n;

  draw(n);
  
}