# hs-searcher

Search budgets and deterministic index strategies for Hankel checkpoint improvement.

| Strategy | Meaning |
|----------|---------|
| `enumerate` | Walk consecutive indices from the saved cursor |
| `local` | Visit the same forward window, closest to the current best index first |
| `sample` | Visit the same forward window in a seed-stable pseudo-random order |

All three strategies advance the checkpoint cursor across the same forward window for `zeta-3` today. They differ only in evaluation order.
