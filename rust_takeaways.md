# Rust Takeaways

> Breakthroughs and insights while learning Rust — Exercism and beyond.

---

### HashMap counting: `entry().or_insert()` to count, `.get()` to check

**Date:** 2026-11-20
**Context:** magazine-cutout (Exercism Rust) — word-counting with a `HashMap`
**Tags:** `rust` `hashmap` `idiom` `entry` `option`

You reached for `.entry().or_insert(0)` for *both* the counting loop and the note-checking loop, then wondered if `.get` was "stupid." It isn't — the two loops want different tools. `entry(word).or_insert(0)` is the canonical "add to a counter that might not exist yet" idiom, so it's the right tool for *counting* the magazine. But for *checking* the note, `.get(word)` is cleaner: it's a pure read, whereas `entry().or_insert(0)` on a missing key *inserts a 0 into the map* — pointless mutation you're about to throw away. Use `entry` to build, `get`/`get_mut` to read.

> **Key takeaway:** Count with `*map.entry(k).or_insert(0) += 1` (the canonical counter idiom); read with `map.get(k)` — `or_insert` mutates on a miss, so don't use it for read-only checks.

---

### `Option` forces you to handle the missing case: `unwrap` vs `unwrap_or`

**Date:** 2026-11-20
**Context:** magazine-cutout (Exercism Rust) — understanding why `get` needs an escape hatch
**Tags:** `rust` `option` `unwrap` `error-handling`

`HashMap::get` returns `Option<&V>` — either `Some(v)` or `None` — because the key might not exist, and Rust forces you to decide what "missing" means *at the call site*. `unwrap()` says "I'm certain it's `Some`; crash loudly if I'm wrong" — convenient but a smell in real code. `unwrap_or(default)` says "or here's a sensible fallback instead of crashing." In the exercise a missing word is *normal* (means "0 copies left"), not a bug, so `*counts.get(word).unwrap_or(&0)` is correct and `unwrap()` is wrong. The `*` at the front just peels off the `&` since `get` hands back a reference.

> **Key takeaway:** `unwrap` = "I'm sure, crash if not" (fine in tests, a smell in production); `unwrap_or(x)` = "use x as a safe fallback" — pick the fallback when "missing" is an ordinary case, not a bug.

---

### nvim: delete every matching line at once with `:g`

**Date:** 2026-11-20
**Context:** Exercism Rust test file — stripping all `#[ignore]` attributes
**Tags:** `nvim` `vim` `cli` `regex`

To remove all `#[ignore]` attributes from a test file, run `:g/#\[ignore\]/d` in normal mode. `:g/pattern/cmd` runs `cmd` on every line matching `pattern`; `d` deletes, so it strips them all in one shot. Brackets must be escaped (`\[` `\]`) so they match literally. Use `print` instead of `d` to preview the matches before deleting.

> **Key takeaway:** `:g/pattern/d` deletes all lines matching `pattern` at once (here `:g/#\[ignore\]/d`) — escape brackets as `\[ \]` for literal matches.
