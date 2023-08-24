# zeta-2

`ζ(2)` 无理性验证：Rust Hankel 计算链 → `hs-lean-helper` 导出 Lean 证书 → `lean/` 形式化验证。

## 架构

```text
hs-moments (ζ(2) 矩) → hs-types (Ferguson P/Q) → zeta-2 (导出证书) → lean/ (验证)
```

Rust 侧复用 Ferguson (arXiv:2003.10616) 与 Prévost Padé/Hankel 路线；Lean 侧形式化：

1. Hankel 矩序列与 Ferguson 行列式定义
2. 与 `export-lean-certificates` 生成的有理数证书对齐（`norm_num`）
3. 整数线性型无理性判据
4. `ζ(2)` 主定理（解析衰减步可独立补全）

## 快速开始

### Rust 证书导出

```bash
cargo run --release -p zeta-2 --bin export-lean-certificates
cargo test --release -p zeta-2
```

### Lean（需 [elan](https://github.com/leanprover/elan)）

```bash
cd projects/zeta-2/lean
lake update
lake build
```

Windows 一键安装 elan 后构建：

```powershell
.\scripts\setup-lean.ps1
```

## 文献

- Timothy Ferguson, *Rational Approximations via Hankel Determinants* (2020)
- Marc Prévost, *A new proof of the irrationality of ζ(2) and ζ(3) using Padé approximants* (2002)
- Wadim Zudilin, *A Determinantal Approach to Irrationality* (2016)
