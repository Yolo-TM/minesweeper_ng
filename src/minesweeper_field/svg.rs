use super::MineSweeperField;
use crate::Cell;
use crate::solver::Finding;
use rand::RngExt;
use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{Animate, Group, Path, Rectangle, Text};

#[allow(non_camel_case_types)]
pub enum SVG_Mode {
    Normal,
    RevealRandom(f32),
    RevealSolving(Vec<Finding>),
}

pub trait MineSweeperFieldSvg: MineSweeperField {
    fn to_svg(&self, file_path: &str, creation_mode: SVG_Mode) {
        let dimensions = self.get_dimensions();
        let (width, height, _) = dimensions;

        let document = Document::new()
            .set(
                "viewBox",
                (
                    0,
                    0,
                    CELL_SIZE * width,
                    get_header_size(width, height) + CELL_SIZE * height,
                ),
            )
            .add(create_background(dimensions))
            .add(create_header(dimensions))
            .add(create_grid(dimensions))
            .add(create_cells(dimensions, self, creation_mode));

        svg::save(file_path, &document).unwrap();
    }
}

impl<T: MineSweeperField> MineSweeperFieldSvg for T {}

const CELL_SIZE: u32 = 20;
const MIN_HEADER_SIZE: u32 = 60;
const MAX_HEADER_SIZE: u32 = 200;

fn revealed_bg(cell: &Cell) -> &'static str {
    match cell {
        Cell::Empty => "#ddd",
        _ => "white",
    }
}

fn get_header_size(width: u32, height: u32) -> u32 {
    let field_size = width.max(height);
    (field_size * CELL_SIZE / 3).clamp(MIN_HEADER_SIZE, MAX_HEADER_SIZE)
}

fn create_background(dimensions: (u32, u32, u32)) -> Rectangle {
    let (width, height, _) = dimensions;
    Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", CELL_SIZE * width)
        .set(
            "height",
            get_header_size(width, height) + CELL_SIZE * height,
        )
        .set("fill", "gray")
}

fn create_header(dimensions: (u32, u32, u32)) -> Text {
    let (width, height, mines) = dimensions;
    let svg_width = CELL_SIZE * width;
    let font_size = svg_width / 16;
    Text::new(format!(
        "Width: {}, Height: {}, Mines: {}",
        width, height, mines
    ))
    .set("x", svg_width / 2)
    .set("y", get_header_size(width, height) / 2)
    .set("text-anchor", "middle")
    .set("dominant-baseline", "middle")
    .set("font-size", font_size)
    .set("fill", "black")
}

fn create_grid(dimensions: (u32, u32, u32)) -> Path {
    let (width, height, _) = dimensions;
    let mut grid = Data::new().move_to((0, get_header_size(width, height)));
    for x in 0..=width {
        grid = grid.vertical_line_by(CELL_SIZE * height);
        grid = grid.move_to((CELL_SIZE * (x + 1), get_header_size(width, height)));
    }
    grid = grid.move_to((0, get_header_size(width, height)));
    for y in 0..=height {
        grid = grid.horizontal_line_by(CELL_SIZE * width);
        grid = grid.move_to((0, get_header_size(width, height) + CELL_SIZE * (y + 1)));
    }
    grid = grid.close();
    Path::new()
        .set("fill", "none")
        .set("stroke", "lightgray")
        .set("stroke-width", 0.5)
        .set("d", grid)
}

/// Core cell rendering. Returns a Group containing background rectangles and text.
fn create_cells(
    dimensions: (u32, u32, u32),
    field: &impl MineSweeperField,
    mode: SVG_Mode,
) -> Group {
    let (w, h, _) = dimensions;
    let mut group = Group::new();

    let mut step_map: std::collections::HashMap<(u32, u32), usize> =
        std::collections::HashMap::new();
    if let SVG_Mode::RevealSolving(ref findings) = mode {
        for (step_idx, finding) in findings.iter().enumerate() {
            // safe fields
            for &pos in finding.get_safe_fields() {
                step_map.entry(pos).or_insert(step_idx);
            }
            // mine fields
            for &pos in finding.get_mine_fields() {
                step_map.entry(pos).or_insert(step_idx);
            }
            // recursive informations
            for inner in finding.get_recursive_revelations() {
                for &pos in inner {
                    step_map.entry(pos).or_insert(step_idx);
                }
            }
        }
    }

    let mut rng = rand::rng();
    for x in 0..w {
        for y in 0..h {
            let cell = field.get_cell(x, y);

            let pos = (CELL_SIZE * x, get_header_size(w, h) + CELL_SIZE * y);
            let rect_id = format!("r_{}_{}", x, y);
            // Base dark rectangle (always added)
            let mut rect = Rectangle::new()
                .set("x", CELL_SIZE * x)
                .set("y", pos.1)
                .set("width", CELL_SIZE)
                .set("height", CELL_SIZE)
                .set("fill", "#222")
                .set("id", rect_id);
            // Determine if this cell should animate to light background
            let (should_animate, delay) = match mode {
                SVG_Mode::Normal => (false, 0.0),
                SVG_Mode::RevealRandom(factor) => {
                    let d = rng.random_range(0.0..(w * h) as f32 * factor);
                    (true, d)
                }
                SVG_Mode::RevealSolving(_) => {
                    if let Some(&step) = step_map.get(&(x, y)) {
                        (true, step as f32 * 0.05)
                    } else {
                        (false, 0.0)
                    }
                }
            };

            if should_animate {
                let anim = Animate::new()
                    .set("attributeName", "fill")
                    .set("from", "#222")
                    .set("to", revealed_bg(cell))
                    .set("begin", format!("{}s", delay))
                    .set("dur", "0.1s")
                    .set("fill", "freeze");
                rect = rect.add(anim);
                // Text appears after background animation
                let mut txt = create_cell_tspan(pos, cell);
                txt = txt.set("visibility", "hidden");
                let txt_anim = Animate::new()
                    .set("attributeName", "visibility")
                    .set("from", "hidden")
                    .set("to", "visible")
                    .set("begin", format!("{}s", delay + 0.1))
                    .set("dur", "0.1s")
                    .set("fill", "freeze");
                txt = txt.add(txt_anim);
                group = group.add(rect);
                group = group.add(txt);
            } else {
                // Normal mode: add both rect and text
                group = group.add(rect);
                group = group.add(create_cell_tspan(pos, cell));
            }
        }
    }
    group
}

fn create_cell_tspan(position: (u32, u32), cell: &Cell) -> Text {
    let cell_text = match cell {
        Cell::Number(num) => num.to_string(),
        Cell::Mine => "💣".to_string(),
        Cell::Empty => "\u{00A0}".to_string(),
    };
    let color = match cell {
        Cell::Mine => "white",
        Cell::Number(num) => match num {
            1 => "blue",
            2 => "green",
            3 => "red",
            4 => "purple",
            5 => "yellow",
            6 => "cyan",
            7 => "black",
            8 => "white",
            _ => unreachable!(),
        },
        Cell::Empty => "transparent",
    };
    Text::new(cell_text)
        .set("x", position.0 as f32 + CELL_SIZE as f32 * 0.5)
        .set("y", position.1 as f32 + CELL_SIZE as f32 * 0.5)
        .set("text-anchor", "middle")
        .set("dominant-baseline", "middle")
        .set("font-size", CELL_SIZE as f32 * 0.8)
        .set("fill", color)
}
