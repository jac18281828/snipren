# `rn` — A Fast, Safe, Intent-Aware Rename Utility

`rn` is a small command-line tool for renaming files by inference.  
You provide the **new name**, and `rn` determines the **old name** — safely, predictably, and fast.

It streamlines common workflows like:
- Changing file extensions: `data.txt` → `data.csv`
- Adding tags or versions: `file.txt` → `file_backup.txt`
- Creating backups: `Makefile` → `Makefile.bak`

```bash
$ ls
route_report.csv

$ rn route_report_before.csv
route_report.csv → route_report_before.csv
```

---

## How `rn` Works

`rn` uses two complementary matching strategies to infer which file you want to rename:

### 1. Extension Change

Matches when the base name is identical but the extension differs.

**Examples:**

```bash
$ rn data.csv
data.txt → data.csv

$ rn report.md
report.txt → report.md

$ rn all_aero_pools.csv
all_aero_pools.txt → all_aero_pools.csv
```

**Rules:**
- Both files must have an extension (contain a dot)
- Everything before the last dot must match exactly
- Only the extension part differs

### 2. Expansion

Matches when characters are added in the middle or end of the filename.

**Examples:**

```bash
$ rn route_report_before.csv
route_report.csv → route_report_before.csv

$ rn data_backup.json
data.json → data_backup.json

$ rn Makefile.bak
Makefile → Makefile.bak

$ rn config~
config → config~
```

**Rules:**
- The new name must be longer than the old name
- All characters from the old name must appear in order (prefix + suffix match)
- Characters are added in the middle or at the end
- Expansion at the start is not allowed (prevents ambiguous matches)

### Safety Requirements

For **any** rename to succeed:
- **Exactly one file must match** — no guessing among multiple candidates
- **Target must not exist** — unless `--force` is used
- **At least one pattern must match** — extension change OR expansion

---

## Examples

### Extension Changes

```bash
$ rn data.csv
data.txt → data.csv

$ rn image.jpg
image.png → image.jpg

$ rn config.yaml
config.yml → config.yaml
```

### Adding Tags or Versions

```bash
$ rn image_before.png
image.png → image_before.png

$ rn results_v2.csv
results.csv → results_v2.csv

$ rn report_final.txt
report.txt → report_final.txt
```

### Backup Patterns

```bash
$ rn Makefile.bak
Makefile → Makefile.bak

$ rn config~
config → config~

$ rn script.sh.bak
script.sh → script.sh.bak
```

### Bidirectional Renames

```bash
# Expansion
$ rn route_report_before.csv
route_report.csv → route_report_before.csv

# Reduction (reverse expansion)
$ rn route_report.csv
route_report_before.csv → route_report.csv
```

---

## Ambiguous Cases & Limitations

### ⚠️ Multiple Matches (Refused)

When multiple files could match, `rn` refuses to guess:

```bash
$ ls
report.csv  report.txt  report.md

$ rn report.json
Multiple candidates found for 'report.json':
  report.csv
  report.txt
  report.md

Cannot proceed - ambiguous match.
```

**Solution:** Be more specific or rename manually with `mv`.

### ⚠️ Similar Extensions (May Match Both Patterns)

Some renames match both extension change AND expansion:

```bash
$ rn config.yaml
# Matches extension change: config.(yml) → config.(yaml)
# Also matches expansion: config.yml → config.yaml (yml is prefix of yaml)
# Result: Works fine! (we use OR logic)
```

This is harmless — the rename succeeds either way.

### ⚠️ No Prefix Expansion

`rn` does NOT match when adding a prefix:

```bash
$ ls
test.log

$ rn production_test.log
No matching files found for 'production_test.log'
# Rejected: expansion at the start is not allowed
```

**Rationale:** Prevents ambiguous matches like `data.json` matching `metadata.json`.

### ⚠️ Files Without Extensions

Extension change requires both files to have extensions:

```bash
$ ls
README

$ rn README.md
README → README.md  # Works! (expansion pattern)

$ ls
Makefile  LICENSE

$ rn LICENSE
Makefile → LICENSE  # FAILS (different base, no match)
```

### ⚠️ Overlapping Names

Be careful with files that are substrings of each other:

```bash
$ ls
test.txt  testing.txt

$ rn test.csv
test.txt → test.csv  # Works

$ rn testing.csv
testing.txt → testing.csv  # Works

$ rn tester.txt
# May match test.txt via expansion (test → tester)
```

**Best Practice:** In directories with many similar names, verify the match before confirming.

---

## Options

```bash
rn <new_name> [OPTIONS]

OPTIONS:
  -f, --force    Force rename even if target exists (overwrites)
  -h, --help     Print help information
```

---

## Safety Guarantees

- **No wild guessing:** Requires exactly one matching file
- **No overwrite:** Refuses if the target already exists (unless `--force`)
- **No ambiguity:** Prints all candidates and exits when multiple files match
- **Atomic rename:** Uses the OS rename syscall for safe file operations

`rn` is designed for interactive use and safety, not for risky batch scripts.

---

## Installation

```bash
cargo install snipren
```

Or build from source:

```bash
git clone https://github.com/jac18281828/snipren
cd snipren
cargo build --release
cp target/release/rn ~/.local/bin/  # or anywhere in your $PATH
```

---

## Why `rn`?

- **Faster than `mv`:** No need to type both the old and new names
- **Intent-aware:** Understands common rename patterns (extensions, tags, backups)
- **Safe by default:** Won't rename if there's any ambiguity
- **Simple:** Just type the name you want, `rn` figures out the rest

**`mv` requires you to know both names.**  
**`rn` lets you rename using the name you want, not the name you must type.**  
**Fast for humans, safe by design.**

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

See [LICENSE](LICENSE) file for details.

