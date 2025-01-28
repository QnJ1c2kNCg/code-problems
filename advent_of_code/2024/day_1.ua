# Experimental!

~ "git: github.com/amatgil/steal-gift" ~ AoCGet

Input ← AoCGet 2024 1

# $ 3   4
# $ 4   3
# $ 2   5
# $ 1   3
# $ 3   9
# $ 3   3
# Input ←

Parse ← ⊜(⊜⋕⊸≠@ )⊸≠@\n

Part₁ ← /+⌵/-≡⍆⍉ Parse

Part₁ Input
