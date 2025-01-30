~ "git: github.com/amatgil/steal-gift" ~ AoCGet

Input ← AoCGet 2024 2

$ 7 6 4 2 1
$ 1 2 7 8 9
$ 9 7 6 2 1
$ 1 3 2 4 5
$ 8 6 4 4 1
$ 1 3 6 7 9
# $ 1 3 6 7 
Sample ←

Parse ← ⊜(⊜⋕≠@ .)≠@\n.
# ⊜(□⊜⋕⊸≠@ )⊸≠@\n

IsAllowedGaps ← ≡/↧(↧⊃(>0)(<4)⌵≡⧈-)
IsSorted      ← ↥⊃(≡/↧=≡(⊏⊸⍏))(≡/↧=≡(⊏⊸⍖)).

# IsSorted Parse Sample
# # Parse Sample
# # ≡⧈-.
# # ↧⊃(>0)(<4)⌵.
# # ≡/↧
# IsAllowedGaps Parse Sample
# /+↧

Part₁ ← /+↧⊃IsSorted IsAllowedGaps
Part₁ Parse Sample
