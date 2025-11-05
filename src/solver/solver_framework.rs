use super::{MineSweeperCellState, SolverSolution, SolverStepCounter, SolverStep};
use crate::*;
use colored::Colorize;

impl<M> MineSweeperSolver<M>
where
    M: MineSweeperField,
{
    pub fn new(field: M) -> Self {
        let state = vec![
            vec![MineSweeperCellState::Hidden; field.get_height() as usize];
            field.get_width() as usize
        ];

        MineSweeperSolver {
            state,
            flag_count: 0,
            hidden_count: (field.get_width() * field.get_height()),
            remaining_mines: field.get_mines(),
            field,
            solution: SolverSolution::NeverStarted,
            steps: SolverStepCounter::new(),
        }
    }

    pub fn start(&mut self, enable_output: bool) -> SolverSolution {
        if enable_output {
            println!(
                "{}: Starting solver with field size {}x{} and {} mines.",
                "Solver started".bold(),
                self.field.get_width(),
                self.field.get_height(),
                self.field.get_mines()
            );
            self.field.show();
            println!(
                "Revealing Start field: ({}, {})",
                self.field.get_start_cell().0,
                self.field.get_start_cell().1
            );
        }

        self.reveal_field(
            self.field.get_start_cell().0,
            self.field.get_start_cell().1,
        );

        return self.continue_solving(enable_output);
    }

    pub fn continue_solving(&mut self, enable_output: bool) -> SolverSolution {
        loop {
            if self.is_solved() {
                if enable_output {
                    println!(
                        "{}: Took {} steps.",
                        "Solver finished".bold(),
                        self.steps.get_pretty_steps()
                    );
                }

                self.flag_all_hidden_cells();
                self.solution = SolverSolution::FoundSolution(self.steps.clone());
                return self.solution.clone();
            } else if enable_output {
                println!("{}: {}", "Solver Step", self.steps.get_pretty_steps());
                self.print();
            }

            match self.do_solving_step() {
                Some(logic_level) => {
                    if enable_output {
                        println!(
                            "{}: Applied insights from {}",
                            "Solver".bold(),
                            logic_level.to_string()
                        );
                    }

                    self.steps.increment(logic_level);
                }
                None => {
                    if enable_output {
                        println!(
                            "{}: No further logic could be applied.",
                            "Solver".bold()
                        );
                    }

                    self.solution = SolverSolution::NoSolution(
                        self.steps.get_steps(),
                        self.remaining_mines,
                        self.hidden_count,
                        self.state.clone(),
                    );
                    return self.solution.clone();
                }
            }

            self.steps.increase_steps();
        }
    }

    fn is_solved(&self) -> bool {
        self.hidden_count == 0 || (self.flag_count + self.hidden_count) == self.field.get_mines()
    }

    pub(super) fn print(&self) {
        for (x, y) in self.field.sorted_fields() {
            match self.get_state(x, y) {
                MineSweeperCellState::Hidden => print!("? "),
                MineSweeperCellState::Flagged => print!("{} ", "F".red()),
                MineSweeperCellState::Revealed => match self.field.get_cell(x as u32, y as u32) {
                    Cell::Empty => print!("  "),
                    Cell::Mine => print!("{} ", "X".red()),
                    Cell::Number(_n) => {
                        print!("{} ", self.field.get_cell(x as u32, y as u32).get_colored())
                    }
                },
            }

            if x == self.field.get_width() - 1 {
                println!();
            }
        }
    }

    pub(super) fn get_state(&self, x: u32, y: u32) -> MineSweeperCellState {
        self.state[x as usize][y as usize].clone()
    }

    fn set_state(&mut self, x: u32, y: u32, state: MineSweeperCellState) {
        self.state[x as usize][y as usize] = state;
    }

    // Method for changing cells at NoGuess Generation
    pub(crate) fn update_field(&mut self, changes: Vec<(u32, u32, Cell)>) {
        for (x, y, state) in changes {
            self.field.set_cell(x, y, state);
        }
    }

    // Method for changing states at NoGuess Generation
    pub(crate) fn update_states(&mut self, changes: Vec<(u32, u32, MineSweeperCellState)>) {
        for (x, y, state) in changes {
            self.set_state(x, y, state);
        }
    }

    fn apply_infos(&mut self, fields: (Vec<(u32, u32)>, Vec<(u32, u32)>)) -> bool {
        let (safe_fields, mine_fields) = fields;
        let mut did_something = false;

        if !safe_fields.is_empty() {
            for (x, y) in safe_fields {
                self.reveal_field(x, y);
            }
            did_something = true;
        }
        if !mine_fields.is_empty() {
            for (x, y) in mine_fields {
                self.flag_cell(x, y);
            }
            did_something = true;
        }

        did_something
    }

    fn do_solving_step(&mut self) -> Option<SolverStep> {

        if self.apply_infos(self.do_basic_neighbour_check()) {
            return Some(SolverStep::Basic);
        }

        if self.apply_infos(self.apply_basic_box_logic()) {
            return Some(SolverStep::Reduction);
        }

        if self.apply_infos(self.apply_extended_box_logic()) {
            return Some(SolverStep::Complex);
        }

        //if self.apply_infos(self.apply_permutation_checks()) {
        //    return Some(SolverStep::Permutations);
        //}

        None
    }

    fn flag_all_hidden_cells(&mut self) {
        for (x, y) in self.field.sorted_fields() {
            if self.get_state(x, y) == MineSweeperCellState::Hidden {
                self.flag_cell(x, y);
            }
        }
    }

    #[track_caller]
    pub(crate) fn reveal_field(&mut self, x: u32, y: u32) {
        if self.get_state(x, y) == MineSweeperCellState::Revealed {
            return;
        }

        self.set_state(x, y, MineSweeperCellState::Revealed);
        self.hidden_count -= 1;

        match self.field.get_cell(x as u32, y as u32) {
            Cell::Mine => {
                panic!("Game Over! The Solver hit a mine at ({}, {})", x, y);
            }
            Cell::Empty => {
                self.reveal_surrounding_cells(x, y);
            }
            Cell::Number(i) => {
                if i == self.get_surrounding_flag_count(x, y) {
                    self.reveal_surrounding_cells(x, y);
                }
            }
        }
    }

    pub(super) fn flag_cell(&mut self, x: u32, y: u32) {
        if self.get_state(x, y) != MineSweeperCellState::Hidden {
            return;
        }

        self.set_state(x, y, MineSweeperCellState::Flagged);
        self.flag_count += 1;
        self.hidden_count -= 1;
        if self.remaining_mines > 0 {
            self.remaining_mines -= 1;
        }
    }

    #[track_caller]
    pub(super) fn reveal_surrounding_cells(&mut self, x: u32, y: u32) {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                self.reveal_field(new_x, new_y);
            }
        }
    }

    pub(super) fn has_unrevealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                return true;
            }
        }

        false
    }

    pub(super) fn has_revealed_neighbours(&self, x: u32, y: u32) -> bool {
        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Revealed {
                return true;
            }
        }

        false
    }

    pub(super) fn get_surrounding_flag_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Flagged {
                count += 1;
            }
        }

        count
    }

    pub(super) fn get_surrounding_unrevealed_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                count += 1;
            }
        }

        count
    }

    pub(super) fn get_surrounding_unrevealed(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut hidden = vec![];

        for (new_x, new_y) in self.field.surrounding_fields(x, y, None) {
            if self.get_state(new_x, new_y) == MineSweeperCellState::Hidden {
                hidden.push((new_x, new_y));
            }
        }

        hidden
    }

    pub(super) fn get_reduced_count(&self, x: u32, y: u32) -> u8 {
        let flag_count = self.get_surrounding_flag_count(x, y);
        let number = self.field.get_cell(x as u32, y as u32).get_number();

        if flag_count > number {
            panic!(
                "Flag count is greater than number at ({}, {}) Flagcount: {}\t Number: {}",
                x, y, flag_count, number
            );
        }

        number - flag_count
    }

    pub(super) fn has_informations(&self, x: u32, y: u32) -> bool {
        self.get_state(x, y) == MineSweeperCellState::Revealed
        && matches!(self.field.get_cell(x, y), Cell::Number(_))
        && self.has_unrevealed_neighbours(x, y)
    }

    pub(super) fn do_basic_neighbour_check(&self) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
        let mut safe_fields = vec![];
        let mut mine_fields = vec![];

        for (x, y) in self.field.sorted_fields() {
            if self.has_informations(x, y) {
                let needed_mines = self.get_reduced_count(x, y);
                let fields = self.get_surrounding_unrevealed(x, y);

                if needed_mines == 0 {
                    safe_fields.extend(fields);
                } else if needed_mines == fields.len() as u8 {
                    mine_fields.extend(fields);
                }
            }
        }

        (safe_fields, mine_fields)
    }
}