# Minesweeper NG Field Generator

## In Work

### Solver

- add more steps for solving
- add default pattern fields for testing
- start with the biggest island with border informations

- new algorithm which gets all disjunct border fields instead of the islands for the permutation checks

- implement a tree for permutation solving?
- improve perm algorithm so in places where only one info field is accessing a part of the border, their permutations are not all tryed bc it doesnt bring value, only check with all count arrangements

- ignore inaccessible parts of the field until nothing else can be solved, then try to solve the inaccessible parts via minecount

### NG Gen

- only mark changed parts as hidden
- implement changing parts of the field to make it solvable

### Other

- allow a 0 tile from every island to be the start tile, so not only one start tile per field is possible (NoGG & Normal Gen)
- Field to SVG
  - animated svg of solving steps / progress?

## Other Solvers

- [mrgris](https://mrgris.com/projects/minesweepr/demo/player/)
- [JS Minesweeper](https://davidnhill.github.io/JSMinesweeper/index.html)
- [Logigames](https://www.logigames.com/minesweeper/solver)
- [Java Solver](missing github link)
