# hs-tools

Installable `hs` command for Hankel search checkpoints.

```bash
cargo install --path projects/hs-tools
hs doctor
hs targets
hs status zeta-3
hs improve zeta-3 --steps 3 --series-terms 256
hs improve zeta-3 --strategy local --steps 3
hs improve zeta-3 --time 30m --write-on-improvement
hs improve zeta-5 --polynomial --steps 1
hs improve zeta-5 --polynomial --steps 1 --full-delta --time 30m
hs check zeta-3
hs verify zeta-3
```

`improve` and `check` operate on finite-index Ferguson bounds under `projects/hs-problems/checkpoints/<target>/checkpoint.json`.
For `zeta-5`, add `--polynomial` to run the Zeta5 paper Hankel construction (`K=40n`, `N=3n`, `h=37n`). Add `--full-delta` only when you intentionally want exact rational `Δ_K` (not a default CI or quick check).
`hs targets` lists every registered checkpoint target. Ferguson search is available for `zeta-2`, `zeta-3`, `zeta-5`, and `zeta-7`.
`--time` accepts suffixes `s`, `m`, and `h`. `--strategy` accepts `enumerate`, `local`, or `sample`. `--jobs` runs independent index evaluations in parallel while preserving merge order.
`--write-on-improvement` skips writing the JSON when only coverage advances without a bound improvement.
`verify` validates registered `rational_equality` proofs and exits with code `2` when no verifier is registered.
`doctor` reports repository root, every checkpoint target readability, and toolchain availability.
`search.benchmark` records the last successful `hs improve` wall time. `hs status` reports it as `workload_eta` and does not treat it as proof discovery time.
