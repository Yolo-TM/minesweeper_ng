use minesweeper_ng_gen::Mines;

#[derive(Clone)]
pub struct Params {
    pub width: String,
    pub height: String,
    pub mines: String, // number or "0.20" for density
    pub use_density: bool,
    pub no_guess: bool,
    pub count: String,
    pub output: String,
}

impl Params {
    pub fn default() -> Self {
        Self {
            width: "30".into(),
            height: "16".into(),
            mines: "99".into(),
            use_density: false,
            no_guess: true,
            count: "1".into(),
            output: String::new(),
        }
    }

    pub fn mine_spec(&self) -> Option<Mines> {
        if self.use_density {
            self.mines.parse::<f32>().ok().map(Mines::Density)
        } else {
            self.mines.parse::<u32>().ok().map(Mines::Count)
        }
    }

    pub fn width_val(&self) -> Option<u32> {
        self.width.parse().ok()
    }
    pub fn height_val(&self) -> Option<u32> {
        self.height.parse().ok()
    }
    pub fn count_val(&self) -> Option<u32> {
        self.count.parse().ok()
    }

    pub fn validate(&self) -> Option<String> {
        let w = self.width_val()?;
        let h = self.height_val()?;
        let mines = self.mine_spec()?;
        let count = self.count_val()?;
        if w < 3 || h < 3 || count == 0 {
            return None;
        }
        if !mines.is_valid(w, h) {
            return None;
        }
        Some(self.resolved_output(w, h, &mines))
    }

    pub fn resolved_output(&self, w: u32, h: u32, mines: &Mines) -> String {
        if !self.output.is_empty() {
            return self.output.clone();
        }
        let prefix = if self.no_guess { "ng_" } else { "" };
        format!(
            "{}{}x{}_{}_mines",
            prefix,
            w,
            h,
            mines.get_fixed_count(w, h)
        )
    }

    pub fn cli_command(&self) -> Option<String> {
        let w = self.width_val()?;
        let h = self.height_val()?;
        let mines = self.mine_spec()?;
        let count = self.count_val()?;
        if !mines.is_valid(w, h) || count == 0 || w < 3 || h < 3 {
            return None;
        }
        let output = self.resolved_output(w, h, &mines);
        let mine_arg = if self.use_density {
            format!("-p {}", self.mines)
        } else {
            format!("-m {}", self.mines)
        };
        let ng_flag = if self.no_guess { " --no-guess" } else { "" };
        let subcommand = if count == 1 { "generate" } else { "batch" };
        let count_arg = if count == 1 {
            String::new()
        } else {
            format!(" -c {}", count)
        };
        Some(
            format!(
                "field_generator {} -w {} -h {}{} -o \"{}\"{}",
                subcommand, w, h, ng_flag, output, count_arg
            ) + &format!(" {}", mine_arg),
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Field {
    Width,
    Height,
    Mines,
    UseDensity,
    NoGuess,
    Count,
    Output,
}

pub const FIELDS: &[Field] = &[
    Field::Width,
    Field::Height,
    Field::Mines,
    Field::UseDensity,
    Field::NoGuess,
    Field::Count,
    Field::Output,
];

pub fn field_label(f: Field) -> &'static str {
    match f {
        Field::Width => "Width",
        Field::Height => "Height",
        Field::Mines => "Mines / Density",
        Field::UseDensity => "Use density (toggle)",
        Field::NoGuess => "No-guess (toggle)",
        Field::Count => "Count",
        Field::Output => "Output dir (blank = auto)",
    }
}

pub fn field_value(f: Field, p: &Params) -> String {
    match f {
        Field::Width => p.width.clone(),
        Field::Height => p.height.clone(),
        Field::Mines => p.mines.clone(),
        Field::UseDensity => {
            if p.use_density {
                "yes".into()
            } else {
                "no".into()
            }
        }
        Field::NoGuess => {
            if p.no_guess {
                "yes".into()
            } else {
                "no".into()
            }
        }
        Field::Count => p.count.clone(),
        Field::Output => p.output.clone(),
    }
}

pub fn is_toggle(f: Field) -> bool {
    matches!(f, Field::UseDensity | Field::NoGuess)
}

pub fn get_field_str_mut<'a>(params: &'a mut Params, f: Field) -> &'a mut String {
    match f {
        Field::Width => &mut params.width,
        Field::Height => &mut params.height,
        Field::Mines => &mut params.mines,
        Field::Count => &mut params.count,
        Field::Output => &mut params.output,
        Field::UseDensity | Field::NoGuess => unreachable!(),
    }
}
