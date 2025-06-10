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
