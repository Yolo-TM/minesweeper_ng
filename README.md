# Minesweeper NoGuess-Field Generator & Solver

> AI-Usage Disclaimer: <br>
> This project started as a way to learn Rust. <br>
> It started with no AI usage whatsoever (can be verified with the commit history) <br>
> After release 0.9.0 AI was used as a writing tool; the design was still made by me and the AI written code thoroughly reviewed and refactored.
>> This README is also partially AI generated

![Animated SVG showing the solver revealing cells step by step on a 150x90 no-guess field](./generated/solver_run.svg)

## Overview

A Rust library and set of binaries for generating and solving Minesweeper fields, with a focus on producing **no-guess** fields — fields that are fully solvable by pure logic without any guessing.

- [Installation](#installation)
- [Library](#library)
  - [Field Types](#field-types)
  - [Field API](#field-api-minesweeperfield-trait)
  - [Solver](#solver)
  - [Error Handling](#error-handling)
- [Binaries](#binaries)
- [Output](#output)
- [Roadmap](#roadmap)
- [Other Solvers](#other-solvers)

## Installation

```toml
[dependencies]
minesweeper_ng_gen = { git = "https://github.com/Yolo-TM/minesweeper_ng" }
```

| Feature | Enables | Default |
| --- | --- | --- |
| `cli` | `field_generator` binary, `clap`, `indicatif`, implies `tui` | yes |
| `tui` | `interactive` binary, `ratatui`, `crossterm`, `simple_logger` | yes (via `cli`) |
| `svg` | SVG field rendering | yes |
| `json` | JSON field serialization | yes |

To use the library only (no binaries, no TUI):

```toml
minesweeper_ng_gen = { git = "https://github.com/Yolo-TM/minesweeper_ng", default-features = false }
```

## Library

### Field Types

| Type | Description |
| --- | --- |
| `DefinedField` | A predefined field, loaded from a file or constructed manually |
| `RandomField` | A randomly generated field (may require guessing to solve) |
| `NoGuessField` | A generated field guaranteed to be solvable without guessing |

All types implement the `MineSweeperField` trait. Extension traits provide additional functionality:

| Trait | Feature | Description |
| --- | --- | --- |
| `MineSweeperFieldDisplay` | always | Colored terminal output |
| `MineSweeperFieldFileIO` | always | Load/save fields to files |
| `MineSweeperFieldJson` | `json` | JSON serialization |
| `MineSweeperFieldSvg` | `svg` | SVG rendering |

### Field API (`MineSweeperField` trait)

All field types implement these methods:

`Cell` represents a single cell: `Cell::Empty`, `Cell::Mine`, or `Cell::Number(u8)`.

```rust
fn get_width(&self) -> u32
fn get_height(&self) -> u32
fn get_mines(&self) -> u32
fn get_dimensions(&self) -> (u32, u32, u32)        // (width, height, mines)
fn get_start_cell(&self) -> (u32, u32)
fn get_cell(&self, x: u32, y: u32) -> &Cell
fn set_cell(&mut self, x: u32, y: u32, cell: Cell)
fn get_surrounding_mine_count(&self, x: u32, y: u32) -> u8
fn sorted_fields(&self) -> SortedCells             // iterator over all (x, y) in row-major order
fn surrounding_fields(&self, x: u32, y: u32, range: Option<u8>) -> SurroundingCells  // iterator over neighboring (x, y) within range (default: 1 = direct neighbors)
fn assign_numbers(&mut self)                        // populate Number cells based on mine positions
fn show(&self)                                      // colored terminal output (MineSweeperFieldDisplay)
```

### Solver

The solver only performs actions that are **100% logically safe** — every reveal and every flag it places is guaranteed to be correct. Strategies are ordered from cheapest to most expensive, restarting from the fastest after each successful deduction, so heavier strategies only run when simpler ones are exhausted.

1. **Simple** — flags cells where neighbour count = mine count, if minecount = 0 reveal neighbours which are hidden
2. **Reduction** — subtracts overlapping constraints between adjacent numbered cells
3. **SAT** — encodes remaining constraints as a satisfiability problem and checks which cell states are forced across all valid solutions
4. **MineCount** — when no mines remain, all hidden cells are safe

> The solver does not yet solve all possible solvable fields. Contributions are welcome!

```rust
use minesweeper_ng_gen::{
    NoGuessField, RandomField, Mines, MineSweeperField,
    Solver, Finding, create_solver, is_solvable,
};

// Generate a no-guess field (see Error Handling for possible failures)
let ng_field: NoGuessField = NoGuessField::new(30, 16, Mines::Count(99))?;

// Generate a random field (may require guessing to solve)
let rng_field: RandomField = RandomField::new(30, 16, Mines::Percentage(0.2))?;

// Quick check: is a field solvable without guessing?
let solvable: bool = is_solvable(&ng_field);

// Run the solver manually
let mut solver: Solver = create_solver(&ng_field);
solver.solve();
let solved: bool = solver.is_solved();
println!("{}", solver.format_field_state()); // colored grid showing solver state

// Inspect what the solver found
let steps: Vec<Finding> = solver.get_solving_steps();
for finding in &steps {
    let safe: &Vec<(u32, u32)> = finding.get_safe_fields();
    let mines: &Vec<(u32, u32)> = finding.get_mine_fields();
    let cascaded: &Vec<Vec<(u32, u32)>> = finding.get_recursive_revelations();
}
```

### Error Handling

`FieldError` covers all failure modes:

| Variant | When |
|---|---|
| `InvalidMineConfig` | Mine count or density is out of range for the given dimensions |
| `OutOfBounds` | Cell coordinate access outside field dimensions |
| `InvalidFileData` | Malformed field file, or a loaded field that is not no-guess |
| `IoError` | File read/write failure |
| `SerializationError` | JSON parse/serialize failure |
| `Deadlock` | No-guess generation failed — layout could not be made solvable |

## Binaries

### `field_generator`

Generates single or batches of fields and writes them to `.minesweeper` files.

```sh
# Generate a single no-guess field (30x16, 99 mines)
field_generator generate -w 30 -h 16 -m 99 --no-guess

# Generate using mine density instead of count
field_generator generate -w 30 -h 16 -p 0.2 --no-guess

# Specify output path
field_generator generate -w 30 -h 16 -m 99 -o my_field

# Batch: generate 50 no-guess fields into a folder
field_generator batch -w 30 -h 16 -m 99 --no-guess -c 50 -o output_dir
```

Default output path: `[ng_]<width>x<height>_<mine_count>_mines[/]` (folder for batch, file for single).

Requires feature `cli`.

### `interactive`

A terminal UI for playing generated fields interactively.

```sh
# Load and play an existing field
interactive my_field.minesweeper

# Create and play a new random field (width height)
interactive create 30 16
```

Requires feature `tui`.

## Output

**File I/O** — available on all field types via `MineSweeperFieldFileIO`:

```rust
use minesweeper_ng_gen::{DefinedField, NoGuessField, MineSweeperFieldFileIO};

field.to_file("my_field.minesweeper")?;
let field: DefinedField = DefinedField::from_file("my_field.minesweeper")?;
let field: NoGuessField = NoGuessField::from_file("my_field.minesweeper")?; // also validates solvability
```

Binary format: `u32 width | u32 height | u32 mines | u32 start_x | u32 start_y | bitpacked mine grid` (little-endian, row-major).

**JSON** — via `MineSweeperFieldJson` (feature `json`):

```rust
use minesweeper_ng_gen::{DefinedField, MineSweeperFieldJson};

let json: String = field.as_json();
let field: DefinedField = DefinedField::from_json(&json)?;
```

```json
{
  "width": 30,
  "height": 16,
  "mines": 99,
  "start_x": 5,
  "start_y": 5,
  "mine_positions": [[1, 2], [3, 4]]
}
```

**SVG** — via `MineSweeperFieldSvg` (feature `svg`):

The SVG output is animated — cells flip to reveal their state with configurable timing.

```rust
use minesweeper_ng_gen::{MineSweeperFieldSvg, SVG_Mode, Solver, Finding, create_solver};

// Static field, all cells shown face-down
field.to_svg("output.svg", SVG_Mode::Normal);

// Animated: all cells flip in random order; the f32 controls the timing spread
// (higher = longer total animation duration)
field.to_svg("output.svg", SVG_Mode::RevealRandom(2.0));

// Animated: cells flip in the order the solver discovered them
let mut solver: Solver = create_solver(&field);
solver.solve();
let steps: Vec<Finding> = solver.get_solving_steps();
field.to_svg("output.svg", SVG_Mode::RevealSolving(steps));
```

## Roadmap

- allow any 0-cell from any island to be the start tile (currently only one start tile per field)
- handle inaccessible islands by opening them / reordering the mines so they are solvable via minecount
- field import from SVG

## Other Solvers

- [mrgris](https://mrgris.com/projects/minesweepr/demo/player/)
- [JS Minesweeper](https://davidnhill.github.io/JSMinesweeper/index.html)
- [Logigames](https://www.logigames.com/minesweeper/solver)
