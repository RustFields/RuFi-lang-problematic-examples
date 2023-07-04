# RuFi-lang-problematic-examples
This repository contains some examples of problems that has been found implementing RuFi language.

## Borrowing
- ex00: Here we show a minimal example of borrowing type conflicts within a function.
- ex01: Here we show how the former problem occurs when implementing the nbr function.
- ex02: Here we try to solve the problem by eliminate nested borrowing inside the nbr function.
- ex03: Here, to allow composition between operators, we make the closure of the nbr function to accept the language as an argument.