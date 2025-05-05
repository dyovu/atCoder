#include <bits/stdc++.h>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define _GLIBCXX_DEBUG
// typedef long long ll;
using ll = long long;



vector<bool> seen;
int dfs(vector<vector<int>> &graph, int t) {
    int count = 0;
    seen[t] = 1;
    for (auto next_v : graph[t]) {
        if (seen[next_v]) continue; 
        count += dfs(graph, next_v); 
    }
    return count + 1;
}


int main(){
    int n, m;
    cin >> n >> m;

    if (n != m) {
        cout << "No" << endl;
        return 0;
    }

    vector<vector<int>> graph(n);
    rep(i, m){
        int x, y;
        cin >> x >> y;
        graph[x-1].push_back(y-1);
        graph[y-1].push_back(x-1);
    }

    rep(i, n){
        if (graph[i].size() != 2) {
            cout << "No" << endl;
            return 0;
        }
    }
    int s = graph[0][0];
    int t = graph[0][1];    


    // cout << "s: " << s << "t: " << t << endl;

    seen.assign(n, false);

    int count = dfs(graph, t);
    // cout << "count: " << count << endl;

    if (seen[s] && count==n) cout << "Yes" << endl;
    else cout << "No" << endl;
}


