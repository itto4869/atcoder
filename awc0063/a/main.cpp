#include <bits/stdc++.h>

#include <atcoder/all>
using namespace std;
using namespace atcoder;

using ll = long long;

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  int h, w;
  cin >> h >> w;

  vector<string> grid(h);
  for (int i = 0; i < h; i++) {
    cin >> grid[i];
  }

  vector<pair<int, int>> v;
  for (int i = 0; i < h; i++) {
    for (int j = 0; j < w; j++) {
      if (grid[i][j] == 'T') {
        v.push_back({i + 1, j + 1});
      }
    }
  }

  cout << v.size() << '\n';
  for (auto [r, c] : v) {
    cout << r << ' ' << c << '\n';
  }
  return 0;
}
