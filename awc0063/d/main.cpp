#include <bits/stdc++.h>

#include <atcoder/all>
using namespace std;
using namespace atcoder;

using ll = long long;

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  ll n;
  cin >> n;

  vector<ll> v(n);
  for (auto& e : v)
    cin >> e;
  vector<vector<ll>> dp(n, vector<ll>(n, 0));

  for (int i = 0; i < n; i++)
    dp[i][i] = v[i];
  for (int sz = 1; sz < n; sz++) {
    for (int i = 0; i < n; i++) {
      int j = i + sz;
      if (j >= n)
        break;
      dp[i][j] = max(v[i] - dp[i + 1][j], v[j] - dp[i][j - 1]);
    }
  }

  cout << dp[0][n - 1] << '\n';

  return 0;
}
