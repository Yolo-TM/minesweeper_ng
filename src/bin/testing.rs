use log::{LevelFilter, info};
use minesweeper_ng_gen::*;
use simple_logger::SimpleLogger;

fn main() {
    // To silence verbose modules, use .with_module_level(...):
    // .with_module_level("minesweeper_ng_gen::solver::strategy::sat_solver", LevelFilter::Warn)
    // .with_module_level("minesweeper_ng_gen::solver", LevelFilter::Info)
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        // .with_module_level("minesweeper_ng_gen::solver::strategy::sat_solver", LevelFilter::Warn)
        .init()
        .unwrap();
    let start = std::time::Instant::now();

    //let field: RandomField = RandomField::new(10, 10, Mines::Density(0.24)).unwrap();
    //let field: DefinedField = DefinedField::from_file("generated/testing/benchmarking/31.minesweeper").unwrap();

    //let solved = is_solvable(&field);
    //let field: DefinedField = DefinedField::from_file("generated/testing/extended_box_logic.minesweeper ").unwrap();

    //let mut solved = false;
    //while !(solved) {
    //    let field: RandomField = RandomField::new(150, 90, Mines::Density(0.19)).unwrap();
    //    let mut solver = create_solver(&field);
    //    solver.solve();
    //    solved = solver.is_solved();
    //    let solving_steps = solver.get_solving_steps();
    //    field.to_svg("output.svg", SVG_Mode::RevealSolver(solving_steps));
    //}

    //debug!("Time elapsed: {:?} - Solved: {:?}", start.elapsed(), solved);

    let field = match NoGuessField::new(20, 20, Mines::Density(0.4)) {
        Ok(field) => {
            info!("Created Field in {:?} ", start.elapsed());
            field
        }
        Err(_) => {
            info!("Failed to created Field in {:?} ", start.elapsed());
            panic!("Failed to created No Guess Field");
        }
    };

    info!("Field solved: {:?}", is_solvable(&field));
    field.show();
}
