# Minesweeper NG Field Generator

![Solver Example](./generated/solver_run.svg)

## Other Solvers

- [mrgris](https://mrgris.com/projects/minesweepr/demo/player/)
- [JS Minesweeper](https://davidnhill.github.io/JSMinesweeper/index.html)
- [Logigames](https://www.logigames.com/minesweeper/solver)
- [Java Solver](missing github link)

## Open Tasks

- allow a 0 tile from every island to be the start tile, so not only one start tile per field is possible (NoGG & Normal Gen)
- field from svg

### Solver

- add meta logic solving strategy
  - solve inaccessible parts

### NG Gen

- change inaccessible parts to be also solvable
- only mark changed parts as hidden and dont restart from scratch
