use minesweeper_ng_gen::*;

fn main() {
    let start = std::time::Instant::now();

    let mut successful = 0;
    let mut unseccessful = 0;

    for i in 1..=100 {
        let field: DefinedField = match DefinedField::from_file(&format!("src/generated/testing/benchmarking/{}.minesweeper", i)) {
            Ok(f) => f,
            Err(_) => {
                continue;
            }
        };

        let start = std::time::Instant::now();
        let is_solvable = is_solvable(&field);

        println!("Field {} solved {} in {:?}", i, if is_solvable { "successfully" } else { "unsuccessfully" }, start.elapsed());

        if is_solvable {
            successful += 1;
        } else {
            unseccessful += 1;
        }
    }

    let elapsed = start.elapsed();
    println!("Time elapsed: {:?} Mean Time: {:?}, Successfully: {}", elapsed, elapsed / (successful + unseccessful), successful);
}
