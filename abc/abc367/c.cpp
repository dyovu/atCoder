#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


int n, k;


void printVector(vector<int> vec){
  // cout << "work" << endl;
  // ほんとは動的にベクトルのサイズ取得したほうがいいかも
  // この問題は出力する配列のサイズがnで固定だからこれでも良い
  // int vecSize = vec.size();
  for(int i=0; i<n; i++){
    cout<<vec[i];
    if(i != n-1)cout << ' ';
  }
  cout<<endl;
}


// ①各要素が1以上R(n)以下の長さnの数列の組み合わせを全列挙する関数を作ろう,,,Pythonでいうproduct
// ②もしくは再帰関数で実装する方法でも良い
// 再起的に配列の各要素がが1以上n以下の配列を生成できればproductかけなくても良くね
// まあ汎用的に使うこと考えたら作っといて損ないか、とりあえず配列の生成だけでいいか




vector<int> tempSeq(0);
// maxSeqのとことを定数にすればpythonのproductみたいに使えるかも
void solve(int depth, vector<int> maxSeq){
  if(depth == n){
    int sum = reduce(tempSeq.begin(), tempSeq.end(), 0);
    // cout << "sum : " << sum << endl;
    if(sum%k == 0){
      printVector(tempSeq);
    }
    return ;
  }
  for(int i=1; i<=maxSeq[depth]; i++){
    tempSeq[depth] = i;
    solve(depth+1, maxSeq);
  }
}



int main(){
  cin >> n >> k;
  tempSeq.resize(n);
  vector<int> r(n);
  vector<vector<int>> allSeq(n);

  rep(i, n){
    int k;
    cin >> k;
    r[i] = k;
  }

  solve(0, r);


}