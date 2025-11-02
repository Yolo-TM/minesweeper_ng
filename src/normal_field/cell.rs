use colored::{ColoredString, Colorize};

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Mine,
    Number(u8),
}

impl Cell {
    pub fn get_number(&self) -> u8 {
        match self {
            Cell::Empty => 0,
            Cell::Mine => 9,
            Cell::Number(num) => *num,
        }
    }

    pub fn get_colored(&self) -> ColoredString {
        match self.get_number() {
            0 => " ".white(),
            1 => "1".bright_blue(),
            2 => "2".green(),
            3 => "3".bright_red(),
            4 => "4".bright_purple(),
            5 => "5".yellow(),
            6 => "6".cyan(),
            7 => "7".black(),
            8 => "8".white(),
            9 => "#".black().bold(),
            _ => unreachable!(),
        }
    }
}