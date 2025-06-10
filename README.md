# Minesweeper NG Field Generator

## In Work

- dont restart completely when changing a part, only mark important parts as hidden
- implement single island changer

- write tests for all patterns (minesweeper.online) for the respective solving steps
- new algorithm which gets all disjunct border fields instead of the islands for the permutation checks?
- allow inaccessible parts be completely filled with mines or completely empty, the field just has to be solvable

- solver is currently not able to solve fields at the end if there are multiple islands without information borders
  - add a way to solve this by checking island sizes against minecount etc
