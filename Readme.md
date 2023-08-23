hankel-searcher
===============

Hankel-determinant rational approximation experiments for constants related to irrationality search.

## Crates

| Crate | Role |
|-------|------|
| `hs-types` | Ferguson `P_n/Q_n` construction and exact Hankel determinants |
| `hs-moments` | Moment sequences for `δ`, `γ`, `ζ(k)`, experimental Catalan kernel |
| `hs-benchmark` | Ferguson table regression and Criterion micro-benchmarks |

## Build

```bash
git checkout dev
cargo build --release -p hs-types -p hs-moments -p hs-benchmark
cargo test --release -p hs-benchmark
cargo bench -p hs-benchmark
```

## Literature

- Timothy Ferguson, *Rational Approximations via Hankel Determinants* ([arXiv:2003.10616](https://arxiv.org/abs/2003.10616))
- Wadim Zudilin, *A Determinantal Approach to Irrationality* (Constructive Approximation, 2016)
- Marc Prévost, *A new proof of the irrationality of ζ(2) and ζ(3) using Padé approximants* (2002)
- F. Calegari, V. Dimitrov, Y. Tang, linear independence of `1, ζ(2), L(2,χ_{-3})` (2024)

## Change the initial commit

```shell
git commit --amend --message "🎂 Project initialized!" --date "2023-01-15"
```

## Emoji Comment

| Emoji  | Meaning                      |  
|--------|------------------------------|  
| 🎂     | Project initialized!         |  
| 🎉     | Release new version          |  
| 🧪🔮   | Experimental code            |   
| 🔧🐛🐞 | Bug fix                      |  
| 🔒     | Security fix                 |  
| 🐣🐤🐥 | Add feature                  |  
| 📝🎀   | Documentation                |  
| 🚀     | Performance improve!         |  
| 🚧     | Work in progress             |  
| 🚨     | Test coverage improve!       |  
| 🚥     | CI improve!                  |  
| 🔥🧨   | Remove code or files         |
| 🧹     | Code refactor                |
| 📈     | Add analytics or branch code |
| 🤖     | Automation fix               |
| 📦     | Update dependencies          |
