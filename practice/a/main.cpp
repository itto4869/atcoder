#include <bits/stdc++.h>
using namespace std;

using ll = long long;

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  int a, b, c;
  cin >> a >> b >> c;

  string s;
  cin >> s;

  int ans;
  ans = a + b + c;
  cout << ans << ' ' << s << '\n';

  return 0;
}
