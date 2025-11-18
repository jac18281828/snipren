# `rn` — A Fast, Safe, Intent-Aware Rename Utility

`rn` is a small command-line tool for renaming files by inference.  
You provide the **new name**, and `rn` determines the **old name** — safely, predictably, and fast.

It streamlines common workflows like adding `_before`, `_after`, dates, tags, or variants to filenames without retyping the original name.

```bash
$ ls
route_report.csv

$ rn route_report_before.csv
# renames: route_report.csv → route_report_before.csv
```

---

## How `rn` Works

### 1. Expansion

Expansion happens when the target name begins with the full current name.

**Example:**

```
old: route_report.csv
new: route_report_before.csv
```

**Rules:**
- If `new_name` starts with `old_name` 
- If ending is identical of both files
- If exactly one file matches: rename it.
- If zero or multiple matches: no action; an explanation is printed.

## Safety Guarantees

- **No wild guessing:** requires exactly one match.
- **No overwrite:** refuses if the target already exists (unless `--force` is used).
- **No ambiguity:** prints candidate files and exits when unsure.
- **Atomic rename** using the OS rename syscall.

`rn` is designed for interactive speed and safety, not for risky batch scripts.

---

## Examples

### Add a tag

```bash
$ rn image_before.png
image.png → image_before.png
```

### Add a version

```bash
$ rn results_v2.csv
results.csv → results_v2.csv
```

### Safe reduction

```bash
$ rn report.csv
route_report_before.csv → report.csv   # only candidate
```

### Ambiguous reduction (refused)

```bash
$ rn route.csv
Multiple candidates:
  route_report_before.csv
  route_raw.csv
```

---

## Installation

```bash
cargo install rn
```

Or download binaries from [Releases](https://github.com/yourusername/rn/releases) and place in your `$PATH`.

---

## Why `rn`?

`mv` requires you to know both names.  
`rn` lets you rename using the name you want, not the name you must type.  
**Fast for humans, safe by design.**

