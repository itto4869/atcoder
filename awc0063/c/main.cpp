#include <bits/stdc++.h>

#include <atcoder/all>
using namespace std;
using namespace atcoder;

using ll = long long;

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  ll n, k;
  cin >> n >> k;

  vector<ll> v(n);
  ll s;
  for (int i = 0; i < n; i++) {
    cin >> s;
    v.push_back(s);
  }

  sort(v.rbegin(), v.rend());

  ll a = pow_mod(2, k, 1000000007);

  v[0] = (v[0] * a) % 1000000007;
  ll ans = 0;
  for (auto s : v) {
    ans = (ans + s) % 1000000007;
  }

  cout << ans << '\n';
  return 0;
}
