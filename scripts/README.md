# AtCoder C++ Scripts

Run from the repository root:

```bash
scripts/build.sh practice/a
scripts/run.sh practice/a < practice/a/tests/sample-1.in
scripts/test.sh practice/a
scripts/submit.sh practice/a
scripts/submit.sh --dry-run practice/a
scripts/submit.sh -y practice/a
```

Run from a task directory:

```bash
../../scripts/build.sh
../../scripts/run.sh < tests/sample-1.in
../../scripts/test.sh
../../scripts/submit.sh
../../scripts/submit.sh --dry-run
../../scripts/submit.sh -y
```

Build outputs are written under `target/cpp/`.

`submit.sh` submits directly to AtCoder using the `acc login` session. By default it asks for
confirmation. Use `--dry-run` to check the target and language without submitting, or `-y` to skip
the confirmation prompt.

To override the selected language ID:

```bash
ATCODER_LANGUAGE_ID=6017 scripts/submit.sh practice/a
```

AtCoder Library is available from `lib/ac-library`:

```cpp
#include <atcoder/all>
using namespace atcoder;
```
