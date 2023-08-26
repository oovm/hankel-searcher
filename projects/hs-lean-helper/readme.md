# hs-lean-helper

Rust → Lean 4 certificate export helpers for the Hankel-Ferguson irrationality pipeline.

## Features

- Build integer-scaled certificates from Ferguson `(P_n, Q_n)` in `hs-types`
- Render `Certificates.lean` for `norm_num` / `decide` checking in Lean
- Generic API reusable by downstream crates such as `zeta-2`

## Usage

```rust,no_run
use hs_lean_helper::{CertificateExport, certificates_from_range, write_certificates};
use hs_types::RationalMomentSequence;
use num_bigint::BigInt;
use num_rational::Ratio;
use std::path::Path;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let moments = RationalMomentSequence::from_positive_moments(vec![
    Ratio::from_integer(BigInt::from(1)),
    Ratio::new(BigInt::from(3), BigInt::from(4)),
    Ratio::new(BigInt::from(11), BigInt::from(18)),
    Ratio::new(BigInt::from(25), BigInt::from(36)),
]);
let rows = certificates_from_range(&moments, 0, 0)?;
let config = CertificateExport::zeta2_default();
write_certificates(Path::new("Certificates.lean"), &config, &rows)?;
# Ok(())
# }
```
