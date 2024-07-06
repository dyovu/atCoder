#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG

int main() {
  int n, t, turn =-1;
  cin >> n >> t;

  map<int, int> row{};
  map<int, int> column{};
  map<int, int> cross{};
  cross[0] = 0;
  cross[1] = 0;
  
  for(int i=0; i<t; i++){
    int k;
    cin >> k;
    // cout << "colum:" << (k-1)/n << endl;

    int colIndex = (k - 1) % n;
    int rowIndex = (k - 1) / n; 
     

    if(column.count(colIndex)){
      column[colIndex] += 1;
      if(column[colIndex] ==n){
        turn = i+1;
        break;
      }
    }else{
      column[colIndex] = 1;
    }

    if(row.count(rowIndex)){
      row[rowIndex] += 1;
      if(row[rowIndex] == n){
        turn = i+1;
        break;
      }
    }else{
      row[rowIndex] = 1;
    }

    if(((k-1)%(n-1))==0 && (k != 1 && k!= n*n)){
      cross[1] += 1;
      if(cross[1] == n){
        turn = i+1;
        break;
      }
    }

    if((k%(n+1))==1){
      cross[0] += 1;
      if(cross[0] == n){
        turn = i+1;
        break;
      }
    }
    

  }

  cout << turn << endl;

}