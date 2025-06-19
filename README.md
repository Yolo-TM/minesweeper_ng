# Minesweeper NG Field Generator

## In Work

- only mark changed parts as hidden
- implement changing parts of the field to make it solvable
- only change one thing at a time bc other parts could get solvable later
- start with the biggest island with border informations

- write tests for all patterns (minesweeper.online) for the respective solving steps
- new algorithm which gets all disjunct border fields instead of the islands for the permutation checks?
- new solving step: allow inaccessible parts be completely filled with mines or completely empty, the minecount should help with this at the end

- solver is currently not able to solve fields at the end if there are multiple islands without information borders
  - add a way to solve this by checking island sizes against minecount etc

## Bugs

- extended logic shouldn't be allowed to generate 0..=1 ranges for single fields

```rust
// extended_logic.rs:182
} else if other_only.len() == 1 && new_other_range.start() != new_other_range.end() && *new_other_range.start() != 0 {
```

would fix it

- permutations is not working properly for border islands, as seen below the solver misplaces a flag bc it doesnt consider the other important number for the 3

```bash
Solver Step: 1
? ? ? ? 2 ? ? F ? ? ? ?
? ? ? ? F 2 2 3 ? ? ? ?
? ? ? ? 3 2   1 ? ? ? ?
? ? ? ? F 2 2 3 ? ? ? ? 
? ? ? ? 3 ? ? F ? ? ? ?
? ? ? ? ? ? ? ? ? ? ? ?
Solver: Applied insights from Permutations
Solver Step: 2
Solver Step: 2
? ? ? ? 2 ? ? F F ? ? ?
? ? ? ? F 2 2 3 ? ? ? ?
? ? ? ? 3 2   1 ? ? ? ?
? ? ? ? F 2 2 3 5 ? ? ?
? ? ? ? 3 ? ? F ? ? ? ?
thread 'main' panicked at src\minesweeper_solver\box_logic\basic_logic.rs:33:42:
Game Over! The Solver hit a mine at (6, 0)
```

## Other Solvers

- [mrgris](https://mrgris.com/projects/minesweepr/demo/player/)
- [JS Minesweeper](https://davidnhill.github.io/JSMinesweeper/index.html)
- [Logigames](https://www.logigames.com/minesweeper/solver)
- [Java Solver](missing github link)
