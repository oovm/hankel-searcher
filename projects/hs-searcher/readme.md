# hs-searcher

Search budgets and deterministic index strategies for Hankel checkpoint improvement.

| Strategy | Meaning |
|----------|---------|
| `enumerate` | Walk consecutive indices from the saved cursor |
| `local` | Visit the same forward window, closest to the current best index first |
| `sample` | Visit the same forward window in a seed-stable pseudo-random order |

All three strategies advance the checkpoint cursor across the same forward window. They differ only in evaluation order. Pass `--jobs` to `hs improve` to evaluate indices in parallel while preserving merge order.

The `rational-parameter-v1` contract walks mixed-radix ordinals over `(index, shift)` with shift in `0..=4`.
