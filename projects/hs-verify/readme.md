# hs-verify

Validates `proof` records attached to checkpoint JSON.

- `rational_equality` via `exact-rational-v1`
- `integer_linear_form` via `integer-linear-form-v1` checks `tau`/`sigma` payload shape and reports `mu` upper bound `1 + sigma/tau` when `linear_form_id` matches the checkpoint objective

`linear_form_id: unassigned` remains unsupported.
