#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;

int h, w;
vector<vector<bool>> seen;
vector<vector<char>> vec_ans;

bool isValidCell(int i, int j) {
    return (i >= 0 && i < h && j >= 0 && j < w);
}

void recursion(vector<vector<int>> vec_exit) {
    if (vec_exit.empty()) return; 
    
    vector<vector<int>> vec_next;

    for (int k = 0; k < vec_exit.size(); k++) {
        int i = vec_exit[k][0]; 
        int j = vec_exit[k][1];
        int direction = vec_exit[k][2];
        // 上が0, 右が1, 下が2, 左が3

        if (isValidCell(i-1, j) && !seen[i-1][j]) {
            seen[i-1][j] = true;
            vec_next.push_back({i-1, j, 0}); 
        }
        if (isValidCell(i+1, j) && !seen[i+1][j]) {
            seen[i+1][j] = true;
            vec_next.push_back({i+1, j, 2}); 
        }
        if (isValidCell(i, j-1) && !seen[i][j-1]) {
            seen[i][j-1] = true;
            vec_next.push_back({i, j-1, 3}); 
        }
        if (isValidCell(i, j+1) && !seen[i][j+1]) {
            seen[i][j+1] = true;
            vec_next.push_back({i, j+1, 1}); 
        }

        if (direction == 2) {
            vec_ans[i][j] = '^';
        } else if (direction == 3) {
            vec_ans[i][j] = '>';
        } else if (direction == 0) {
            vec_ans[i][j] = 'v';
        } else if (direction == 1) {
            vec_ans[i][j] = '<';
        }
    }
    
    recursion(vec_next);
}

int main(){
    cin >> h >> w;

    int mega = 1000000; 

    vector<vector<char>> vec_path(h, vector<char>(w, '#'));
    vector<vector<int>> vec_exit;

    vec_ans.assign(h, vector<char>(w, '#'));
    seen.assign(h, vector<bool>(w, false));

    int count_e = 0;

    rep (i, h){
        rep (j, w){
            char c;
            cin >> c;
            if (c == '.'){
                vec_path[i][j] = '.';
            } else if (c == 'E'){
                vec_exit.push_back({i, j, -1}); 
                vec_ans[i][j] = 'E';
                count_e++;
                seen[i][j] = true;
            } else {
                seen[i][j] = true;
            }
        }
    }

    recursion(vec_exit);
    
    // 結果の出力を追加
    rep(i, h) {
        rep(j, w) {
            cout << vec_ans[i][j];
        }
        cout << endl;
    }
    
    return 0;
}