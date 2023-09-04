# hs-tools

Install from the repository root with `cargo install --path projects/hs-tools`.
The installed command is `hs`.

Run `hs improve zeta-3` from the repository root to evaluate the next Ferguson approximant and update `projects/zeta-3/checkpoint.json`. The reported bound is a rigorous finite-index approximation-error bound obtained from a rational interval for `ζ(3)`. It is an observation, not a uniform irrationality proof.

Use `hs check zeta-3` to recompute the saved observation before submitting the JSON change in a pull request.
