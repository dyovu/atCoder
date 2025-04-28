#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int main() {
    vector<int> a = {1, 4, 4, 7, 7, 8, 8, 11, 13, 19};
    set<int> b = {1, 4, 4, 7, 7, 8, 8, 11, 13, 19};

    auto Iter1 = lower_bound(a.begin(),a.end(), 4);
    // 4以上の要素で一番左のイテレータを返す -> index1のイテレーター（最初の4）
    auto Iter2 = lower_bound(a.begin(),a.end(), 1);
    // 6以上の要素で一番左のイテレータを返す -> index3のイテレーター（最初の7）

    auto Iter3 = upper_bound(a.begin(),a.end(), 0);
    // 4より大きいの要素で一番左のイテレータを返す -> index3のイテレーター（最初の7)

    // setに対するlower_bound 
    //引数としてではなくメソッドのように使う、
    auto ite = b.lower_bound(20);
    // cout << *ite << endl;
    // cout << (ite == end(b)) << endl;
    return 0;
}
