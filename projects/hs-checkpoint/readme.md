# hs-checkpoint

Checkpoint JSON schema v2, validation, v1 migration, and atomic writes for Hankel search targets.

Search contracts are registered in `search.rs`. `ferguson-index-v1` + `nonnegative-index-v1` and `ferguson-parameter-v1` + `rational-parameter-v1` are implemented for `hs improve`.

Run one `hs` writer process per machine. In-process parallelism via `--jobs` is supported. Dual-process checkpoint locking is intentionally out of scope.
