import Lake
open Lake DSL

package «zeta-3-lean» where
  leanOptions := #[
    ⟨`autoImplicit, false⟩,
    ⟨`relaxedAutoImplicit, false⟩
  ]

lean_lib Hankel where
  roots := #[`Hankel]

lean_lib Zeta3 where
  roots := #[`Zeta3]
