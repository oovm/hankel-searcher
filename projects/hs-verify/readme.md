# hs-verify



Validates `proof` records attached to checkpoint JSON.



- `rational_equality` via `exact-rational-v1` checks canonical fractions **and** target identity. Zeta checkpoint targets reject certificates that do not match a rigorous series enclosure. The conformance fixture `fixture-half` accepts `1/2` only.

- `integer_linear_form` via `integer-linear-form-v1` remains **unsupported** until decay, coefficient growth, and nondegeneracy evidence are implemented. Payload shape alone never returns `Verified`.



`linear_form_id: unassigned` remains unsupported.


