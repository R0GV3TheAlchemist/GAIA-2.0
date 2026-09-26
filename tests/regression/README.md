# Regression locks (#936)

Executable tests live in `gaia-integrity/tests/regression.rs`.
Run:

```
cargo test -p gaia-integrity --test regression
```

Policy: every PR that closes a `bug` or `defect` issue must add an `rNNNN` lock in that file. See `CONTRIBUTING.md`.
