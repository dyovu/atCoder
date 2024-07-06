#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main(){
  string str_n ;
  unsigned int digit = 0;
  cin >> str_n;
  int k = str_n.length() ;

  for(long long i=0; i < (1 << k); i++){
    string tmp_str = "";
    long long int_n;

    for(long long j=0; j<k; j++){
      if( (i>>j)&1 ){
        tmp_str += str_n.at(j);
      }
    }
  
    if(tmp_str.length() != 0){
      int_n = stoll(tmp_str);
      // int len = static_cast<int>(tmp_str.length());
      unsigned int len = tmp_str.length();
      if((int_n%3 == 0) && (len > digit)){
        digit = len;
      }
    }
  }

  if(digit==0){
    cout << -1 << endl;
  }else{
    cout << k - digit << endl;
  }

}