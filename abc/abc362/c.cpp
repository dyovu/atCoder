#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;
#define _GLIBCXX_DEBUG


void calc(vector<int> &tmpl, vector<int> &tmpr, ll &tmp_suml){
  int cnt=0;
  int n = tmpl.size();

  while(tmp_suml<0 && cnt < n){
    // cout << "tmp_suml" << cnt << tmp_suml << endl;

    if(tmp_suml + tmpr.at(cnt) - tmpl.at(cnt) < 0){
      tmp_suml += tmpr.at(cnt) - tmpl.at(cnt);
      tmpl.at(cnt) = tmpr.at(cnt);
    }else{
      tmpl.at(cnt) -= tmp_suml;
      tmp_suml = 0;
    }
    cnt ++;
  }
  return ;
}

int main(){
  int n;
  ll suml=0, sumr=0;
  cin >> n;

  vector<int> listl(n);
  vector<int> listr(n);

  rep(i , n){
    int l, r;
    cin >> l >> r;
    suml += l;
    sumr += r;
    listl[i] = l;
    listr[i] = r;
  }
  // cout << "n : " << n << endl;

  if(suml<=0 && 0<=sumr){
    calc(listl, listr, suml);
    cout << "Yes" << endl;
    rep(i ,n-1){
      cout << listl[i] << ' ';
    }
    cout << listl[n-1] << endl;
  }else{
    cout << "No" << endl;
  }
}