use crate::field_generator::minesweeper_field::MineSweeperField;
use crate::field_generator::minesweeper_cell::MineSweeperCell;
use super::boxes::Box;
use colored::Colorize;
use core::panic;
use std::{collections::HashMap, hash::Hash, thread, vec};

enum SolverSolution {
    NoSolution,
    FoundSolution,
}

#[derive(Clone, PartialEq)]
enum MineSweeperCellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone)]
struct MineSweeperSolver{
    pub field: MineSweeperField,
    pub state: Vec<Vec<MineSweeperCellState>>,
    pub flag_count: u64,
    pub hidden_count: u64,
    pub remaining_mines: u64
}

impl MineSweeperSolver {
    pub fn new(field : MineSweeperField) -> Self {
        let state = vec![vec![MineSweeperCellState::Hidden; field.height]; field.width];

        MineSweeperSolver {
            state,
            flag_count: 0,
            hidden_count: (field.width * field.height) as u64,
            remaining_mines: field.mines,
            field,
        }
    }

    fn print(&self) {
        for (x, y) in self.field.sorted_fields() {
            match self.state[x][y] {
                MineSweeperCellState::Hidden => print!("? "),
                MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                MineSweeperCellState::Revealed => match self.field.board[x][y] {
                    MineSweeperCell::Empty => print!("  "),
                    MineSweeperCell::Mine => print!("{} ", "X".red()),
                    MineSweeperCell::Number(_n) => print!("{} ", self.field.board[x][y].get_colored()),
                },
            }

            if x == self.field.width - 1 {
                println!();
            }
        }
    }

    fn do_solving_step(&mut self) -> Option<()>{
        //match self.do_basic_neighbour_check(){
        //    Some(_) => {
        //        println!("Revealed or Flagged Fields based on basic count logic.");
        //        return Some(());
        //    },
        //    None => {}
        //}

        //match self.apply_basic_box_logic() {
        //    Some(_) => {
        //        println!("Revealed or Flagged Fields based on box logic.");
        //        return Some(());
        //    },
        //    None => {}
        //}

        match self.apply_permutation_checks() {
            Some(_) => {
                println!("Revealed or Flagged Fields based on tested permutations.");
                return Some(());
            },
            None => {}
        }
        None
    }

    fn flag_all_hidden_cells(&mut self) {
        for (x, y) in self.field.sorted_fields() {
            if self.state[x][y] == MineSweeperCellState::Hidden {
                self.flag_cell(x, y);
            }
        }
    }

    #[track_caller]
    fn reveal_field(&mut self, x: usize, y: usize) {
        if self.state[x][y] == MineSweeperCellState::Revealed {
            return;
        }

        self.state[x][y] = MineSweeperCellState::Revealed;
        self.hidden_count -= 1;

        match self.field.board[x][y] {
            MineSweeperCell::Mine => {
                panic!("Game Over! The Solver hit a mine at ({}, {})", x, y);
            }
            MineSweeperCell::Empty => {
                self.reveal_surrounding_cells(x, y);
            }
            MineSweeperCell::Number(i) => {
                if i == self.get_surrounding_flag_count(x, y) {
                    self.reveal_surrounding_cells(x, y);
                }
            }
        }
    }

    fn flag_cell(&mut self, x: usize, y: usize) {
        if self.state[x][y] == MineSweeperCellState::Revealed || self.state[x][y] == MineSweeperCellState::Flagged {
            println!("Cell ({}, {}) is already revealed or flagged.", x, y);
            return;
        }

        self.state[x][y] = MineSweeperCellState::Flagged;
        self.flag_count += 1;
        self.hidden_count -= 1;
        self.remaining_mines -= 1;
    }

    #[track_caller]
    fn reveal_surrounding_cells(&mut self, x: usize, y: usize) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                self.reveal_field(new_x, new_y);
            }
        }
    }

    fn flag_surrounding_cells(&mut self, x: usize, y: usize) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                self.flag_cell(new_x, new_y);
            }
        }
    }

    fn has_unrevealed_neighbours(&self, x: usize, y: usize) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                return true;
            }
        }

        false
    }

    fn get_surrounding_flag_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Flagged {
                count += 1;
            }
        }

        count
    }

    fn get_surrounding_unrevealed_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                count += 1;
            }
        }

        count
    }

    fn get_surrounding_unrevealed(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut hidden = vec![];

        for (new_x, new_y) in self.field.surrounding_fields(x, y) {
            if self.state[new_x][new_y] == MineSweeperCellState::Hidden {
                hidden.push((new_x, new_y));
            }
        }

        hidden
    }

    fn get_reduced_count(&self, x: usize, y: usize) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = self.field.board[x][y].get_number();

        if flag_count > number {
            panic!("Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}", x, y, flag_count, number);
        }

        number - flag_count
    }

    fn has_informations(&self, x: usize, y: usize) -> bool {
        self.state[x][y] == MineSweeperCellState::Revealed
        && matches!(self.field.board[x][y], MineSweeperCell::Number(_))
        && self.has_unrevealed_neighbours(x, y)
    }

    fn do_basic_neighbour_check(&mut self) -> Option<()> {
        let mut did_something = false;

        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                let needed_mines = self.get_reduced_count(x, y);
                if needed_mines == self.get_surrounding_unrevealed_count(x, y) {
                    self.flag_surrounding_cells(x, y);
                    did_something = true;
                }
                if needed_mines == 0 {
                    self.reveal_surrounding_cells(x, y);
                    did_something = true;
                }
            }
        }

        if did_something {
            return Some(());
        } else {
            return None;
        }
    }

    fn apply_basic_box_logic(&mut self) -> Option<()> {
        let mut did_something = false;

        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                let reduced_count = self.get_reduced_count(x, y);
                let surrounding_hidden_fields = self.get_surrounding_unrevealed(x, y);

                for (new_x, new_y) in self.field.extended_surrounding_fields(x, y, 5) {
                    if self.has_informations(new_x, new_y) {
                        let reduced_count2 = self.get_reduced_count(new_x, new_y);
                        let surrounding_hidden_fields2 = self.get_surrounding_unrevealed(new_x, new_y);

                        let mut shared_fields = vec![];
                        let mut not_shared_fields = vec![];
                        for hidden_cell in &surrounding_hidden_fields2 {
                            if surrounding_hidden_fields.contains(hidden_cell) {
                                shared_fields.push(*hidden_cell);
                            } else {
                                not_shared_fields.push(*hidden_cell);
                            }
                        }

                        if surrounding_hidden_fields.len() == shared_fields.len() {
                            // Found two numbers which share the same unrevealed fields.
                            // Now we can check if we can solve other neighbouring fields of new_x and new_y with this extra informations
                            let reduced_diff = reduced_count2 - reduced_count;

                            if reduced_count == reduced_count2{
                                for cell in &not_shared_fields {
                                    self.reveal_field(cell.0, cell.1);
                                    did_something = true;
                                }
                            } else if reduced_diff == (self.get_surrounding_unrevealed_count(new_x, new_y) - shared_fields.len() as u8) {
                                for cell in &not_shared_fields {
                                    self.flag_cell(cell.0, cell.1);
                                    did_something = true;
                                }
                            }
                        } else if reduced_count > reduced_count2 {
                            let mut rest_fields = vec![];
                            for cell in &surrounding_hidden_fields {
                                if !shared_fields.contains(cell) {
                                    rest_fields.push(*cell);
                                }
                            }
                            let reduced_diff = (reduced_count - reduced_count2) as usize;

                            if reduced_diff == rest_fields.len() {
                                for cell in &rest_fields {
                                    self.flag_cell(cell.0, cell.1);
                                    did_something = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if did_something {
            return Some(());
        } else {
            return None;
        }
    }

    fn apply_permutation_checks(&mut self) -> Option<()> {
        let mut did_something = false;
        let mut permutation_field: HashMap<(usize, usize), u64>  = HashMap::new();
        let mut possible_permutations: u64 = 0;

        // create a map with all fields which we use for permutations
        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                permutation_field.insert((x, y), 0);
            }
        }

        // generate all possible permutations for the fields



        // apply found informations to the map
        for ((x, y), permutation_mines) in permutation_field {
            if permutation_mines == 0 {
                // Field is in every possible way empty
                self.reveal_field(x, y);
                did_something = true;
            }

            if permutation_mines == possible_permutations {
                // Field is in every possible way a mine
                self.flag_cell(x, y);
                did_something = true;
            }
        }

        if did_something {
            return Some(());
        } else {
            return None;
        }
    }
}


pub fn start(field: MineSweeperField) {
    let mut game = MineSweeperSolver::new(field);

    println!("\nSolving the field...");
    println!("Width: {}, Height: {}, Mines: {}", game.field.width.to_string().green(), game.field.height.to_string().green(), game.field.mines.to_string().red());

    let handle = thread::Builder::new()
    .stack_size(32 * 1024 * 1024) // 32 MB
    .spawn(|| {
        let mut step_count: u64 = 0;
        game.reveal_field(game.field.start_field.0, game.field.start_field.1);

        match solver(game, &mut step_count) {
            SolverSolution::NoSolution => {
                println!("No solution found. Stopped after {} steps.", step_count.to_string().red());
            }
            SolverSolution::FoundSolution => {
                println!("Found a solution after {} steps.", step_count.to_string().green());
            }
        }
    })
    .expect("Thread spawn failed");

    handle.join().unwrap();
}

fn solver(mut game: MineSweeperSolver, step_count: &mut u64) -> SolverSolution {
    let mut nothing_count = 0;

    loop {
        (*step_count) += 1;
        println!("Solving Step: {}", step_count.to_string().green());

        if game.hidden_count == 0 {
            println!("All cells revealed. Game solved!");
            game.print();
            return SolverSolution::FoundSolution;
        }

        if (game.flag_count + game.hidden_count) == game.field.mines {
            println!("All non mine cells revealed and all mines flagged. Game solved!");
            game.flag_all_hidden_cells();
            game.print();
            return SolverSolution::FoundSolution;
        }

        match game.do_solving_step() {
            Some(_) => {
                nothing_count = 0;
            },
            None => {
                nothing_count += 1;
                if nothing_count >= 3 {
                    println!("Nothing found in 3 steps. Stopping solver.");
                    game.print();
                    println!("Hidden Count: {}", game.hidden_count.to_string().red());
                    println!("Island Count: {}", search_for_islands(&game).len());
                    //println!("islands: {:?}", search_for_islands(&game));
                    return SolverSolution::NoSolution;
                }
            }
        }
    }
}


// bei mehreren nicht lösbaren islands, Minen von einer Box bei einer Island in eine andere reinpacken und dann schauen ob es lösbar wird

fn search_for_islands(game: &MineSweeperSolver) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; game.field.height]; game.field.width];
    let mut islands = vec![];

    for (x, y) in game.field.sorted_fields() {
        if visited[x][y] || game.state[x][y] != MineSweeperCellState::Hidden {
            continue;
        }

        let mut fields = vec![];
        recursive_search(x, y, &mut fields, &mut visited, game);
        if fields.len() > 0 {
            islands.push(fields);
        }
    }

    islands
}

fn recursive_search(x: usize, y: usize, fields: &mut Vec<(usize,usize)>, visited : &mut Vec<Vec<bool>>, game: &MineSweeperSolver) {
    visited[x][y] = true;
    fields.push((x, y));

    for (new_x, new_y) in game.field.surrounding_fields(x, y) {
        if !visited[new_x][new_y] && game.state[new_x][new_y] == MineSweeperCellState::Hidden {
            recursive_search(new_x, new_y, fields, visited, game);
        }
    }
}
