# hs-lean-helper

Rust → Lean 4 证书导出辅助库，服务于 Hankel-Ferguson 无理性验证链。

## 能力

- 从 `hs-types` 的 Ferguson `(P_n, Q_n)` 构造整数缩放证书
- 渲染 `Certificates.lean` 供 `norm_num` / `decide` 校验
- 通用 API，供 `zeta-2` 等下游 crate 复用

## 用法

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
