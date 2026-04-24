// SVG rendering for Minesweeper fields with reveal animation
use crate::Cell;
use rand::RngExt;
use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{Animate, Path, Rectangle, TSpan, Text};

#[allow(non_camel_case_types)]
pub enum SVG_Mode {
    Normal,
    RevealRandom(f32),
}

const CELL_SIZE: u32 = 20;
const MIN_HEADER_SIZE: u32 = 60;
const MAX_HEADER_SIZE: u32 = 200;

/*
 Output an Image as SVG file

 - Normal Mode just show all
 - Includes per‑cell reveal animation (tiles start hidden and appear randomly)
*/

pub fn create_field(dimensions: (u32, u32, u32), field: &Vec<Vec<Cell>>, file_path: &str, mode: SVG_Mode) {
    let (width, height, _mines) = dimensions;

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
        .add(create_cells(dimensions, field, mode));

    svg::save(file_path, &document).unwrap();
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

fn create_cells(dimensions: (u32, u32, u32), field: &Vec<Vec<Cell>>, mode: SVG_Mode) -> Text {
    let (width, height, _) = dimensions;
    let font_size = CELL_SIZE as f32 * 0.8;
    let mut text = Text::new("")
        .set("x", 0)
        .set("y", get_header_size(width, height))
        .set("text-anchor", "middle")
        .set("dominant-baseline", "middle")
        .set("font-size", font_size);

    let mut rng = rand::rng();
    for x in 0..width {
        for y in 0..height {
            let cell = &field[x as usize][y as usize];
            if let Cell::Empty = cell {
                continue;
            }
            let position = (
                CELL_SIZE * x,
                get_header_size(width, height) + CELL_SIZE * y,
            );

            match mode {
                SVG_Mode::Normal => {
                    text = text.add(create_cell(position, cell));
                }
                SVG_Mode::RevealRandom(delay) => {
                    let mut cell_tspan = create_cell(position, cell);
                    // start hidden
                    cell_tspan = cell_tspan.set("visibility", "hidden");
                    // random delay for reveal (seconds)
                    let delay = rng.random_range(0.0..(width * height) as f32 * delay);
                    let animate = Animate::new()
                        .set("attributeName", "visibility")
                        .set("from", "hidden")
                        .set("to", "visible")
                        .set("begin", format!("{}s", delay))
                        .set("dur", "0.1s")
                        .set("fill", "freeze");
                    cell_tspan = cell_tspan.add(animate);
                    text = text.add(cell_tspan);
                }
            }
        }
    }
    text
}

fn create_cell(position: (u32, u32), cell: &Cell) -> TSpan {
    let cell_text = match cell {
        Cell::Number(num) => num.to_string(),
        Cell::Mine => "💣".to_string(),
        Cell::Empty => unreachable!(),
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
        Cell::Empty => unreachable!(),
    };
    TSpan::new(cell_text)
        .set("x", position.0 as f32 + CELL_SIZE as f32 * 0.6)
        .set("y", position.1 as f32 + CELL_SIZE as f32 * 0.6)
        .set("fill", color)
}
