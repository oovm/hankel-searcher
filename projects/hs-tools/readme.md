# hs-tools

Installable `hs` command for Hankel search checkpoints.

```bash
cargo install --path projects/hs-tools
hs doctor
hs targets
hs status zeta-3
hs improve zeta-3 --steps 3 --series-terms 256
hs improve zeta-3 --time 30m --write-on-improvement
hs check zeta-3
hs verify zeta-3
```

`improve` and `check` operate on finite-index Ferguson bounds in `projects/targets/zeta-3/checkpoint.json`.
`--time` accepts suffixes `s`, `m`, and `h`. `--jobs` above `1` is rejected until parallel search exists.
`--write-on-improvement` skips writing the JSON when only coverage advances without a bound improvement.
`verify` validates registered `rational_equality` proofs and exits with code `2` when no verifier is registered.
`doctor` reports repository root, checkpoint readability, and toolchain availability.
