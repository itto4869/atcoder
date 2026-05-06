# AtCoder C++ Scripts

Run from the repository root:

```bash
scripts/build.sh practice/a
scripts/run.sh practice/a < practice/a/tests/sample-1.in
scripts/test.sh practice/a
scripts/submit.sh practice/a
```

Run from a task directory:

```bash
../../scripts/build.sh
../../scripts/run.sh < tests/sample-1.in
../../scripts/test.sh
../../scripts/submit.sh
```

Build outputs are written under `target/cpp/`.

AtCoder Library is available from `lib/ac-library`:

```cpp
#include <atcoder/all>
using namespace atcoder;
```
