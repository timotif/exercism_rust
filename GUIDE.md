# How to Do an Exercise

A short, offline-friendly workflow for this fork. Pick exercises from [EXERCISES.md](EXERCISES.md).

## 1. Anatomy of an exercise

```
exercises/practice/reverse-string/
├── .docs/
│   ├── introduction.md        ← background / concept explanation
│   ├── instructions.md        ← what to build
│   └── hints.md               ← (concept exercises) only when stuck
├── .meta/example.rs           ← ⚠️ reference solution: open only AFTER you finish
├── Cargo.toml                 ← crate name + dependencies
├── src/lib.rs                 ← ✏️ YOUR code goes here
└── tests/reverse_string.rs    ← the spec: what your code must do
```

> `.docs` and `.meta` start with a dot, so plain `ls` hides them — use `ls -a`.

## 2. Read, in this order

1. `.docs/introduction.md` and `.docs/instructions.md`
2. `tests/<name>.rs` — each `#[test]` is one example of expected behaviour. The `assert_eq!(output, expected)` lines tell you exactly what to return.

## 3. Write the code

Open `src/lib.rs`. You'll find stubs like:

```rust
pub fn reverse(input: &str) -> String {
    todo!("Write a function to reverse {input}");
}
```

- Replace the `todo!(...)` with your implementation.
- **Don't change the signature** (name, parameters, return type) — the tests call it exactly like that.
- Keep it `pub`: the test file is a separate crate that imports your code with `use reverse_string::*;`.
- You may add helper functions, structs, `use` statements, etc.

## 4. Run the tests

From inside the exercise folder:

```sh
cargo test
```

Only the **first** test runs at first; the rest are marked `#[ignore]`. Expected start:

```
panicked at src/lib.rs:2:5:
not yet implemented: ...
test result: FAILED. 0 passed; 1 failed; 6 ignored
```

That's normal — it's the `todo!()` panicking. The loop:

1. Make the current test pass.
2. In `tests/<name>.rs`, delete the `#[ignore]` line above the next test.
3. `cargo test` again. Repeat until no `#[ignore]` is left.

### Handy variants

| Command | What it does |
|---|---|
| `cargo test` | Run non-ignored tests |
| `cargo test -- --include-ignored` | Run **all** tests (final check) |
| `cargo test a_word` | Run only tests whose name contains `a_word` |
| `cargo test -- --nocapture` | Show your `println!` / `dbg!` output |
| `cargo test --offline` | Never touch the network |
| `cargo check` | Just compile (fast) — great while fixing errors |

## 5. Reading errors

- **Compile errors** (`error[E0308]: mismatched types` …): read the **first** one only, fix it, re-run. Later errors are often caused by the first.
- Every error code has an offline explanation: `rustc --explain E0308`
- **Test failures** show `left` (your output) vs `right` (expected).
- Debug by printing: `dbg!(&value);` then `cargo test -- --nocapture`.

## 6. Polish

```sh
cargo fmt        # auto-format
cargo clippy     # idiomatic-Rust suggestions — read them, they teach a lot
```

## 7. Finish

1. `cargo test -- --include-ignored` → all green.
2. Now compare with `.meta/example.rs` — note anything you'd do differently.
3. Tick the box in `EXERCISES.md` and commit:
   ```sh
   git add -A && git commit -m "reverse-string"
   ```

## Offline docs

```sh
rustup doc --book    # The Rust Programming Language
rustup doc --std     # standard library (search box works offline)
rustup doc --rust-by-example
```

Tip: when a test needs a method you don't know (e.g. reversing a string), search the **std docs** for the type you have (`str`, `String`, `Vec`, `Iterator`) and browse its methods.
