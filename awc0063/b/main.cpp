#include <bits/stdc++.h>

#include <atcoder/all>
using namespace std;
using namespace atcoder;

using ll = long long;
using i128 = __int128_t;

i128 read_i128() {
  string s;
  cin >> s;

  i128 x = 0;
  int start = 0;
  bool neg = false;

  if (s[0] == '-') {
    neg = true;
    start = 1;
  }

  for (int i = start; i < (int)s.size(); i++) {
    x = x * 10 + (s[i] - '0');
  }

  return neg ? -x : x;
}

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  i128 n = read_i128();
  i128 m = read_i128();
  i128 p;
  i128 ans = 1;

  bool ok = true;
  for (int i = 0; i < n; i++) {
    p = read_i128();
    ans = lcm(ans, p);
    if (ans > m) {
      ok = false;
      break;
    }
  }

  if (ok) {
    cout << "Yes" << '\n';
  } else {
    cout << "No" << '\n';
  }
  return 0;
}