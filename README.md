# Minesweeper NG Field Generator

## In Work

- add tui / ratatui for better output
- add features
- split the Minesweeper trait into multiple sub traits
- allow a 0 tile from every island to be the start tile, so not only one start tile per field is possible (NoGG & Normal Gen)
- Field to SVG
  - animated svg of solving steps / progress?

### Solver

- add meta logic solving step/s
- start with the biggest island with border informations
- ignore inaccessible parts of the field until nothing else can be solved, then try to solve the inaccessible parts via minecount

### NG Gen

- only mark changed parts as hidden
- implement changing parts of the field to make it solvable

## Other Solvers

- [mrgris](https://mrgris.com/projects/minesweepr/demo/player/)
- [JS Minesweeper](https://davidnhill.github.io/JSMinesweeper/index.html)
- [Logigames](https://www.logigames.com/minesweeper/solver)
- [Java Solver](missing github link)
