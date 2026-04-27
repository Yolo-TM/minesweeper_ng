use super::NoGuessField;
use super::candidate::CandidatePicker;
use super::failed_moves::{FailedDoubleMoves, FailedMoves};
use super::frontier::Frontier;
use crate::solver::create_solver;
use crate::{DefinedField, FieldError, MineSweeperField, Mines, RandomField};

use log::{debug, info};
use rayon::prelude::*;

pub(super) fn generate(
    width: u32,
    height: u32,
    mines: Mines,
    batch_size: usize,
) -> Result<NoGuessField, FieldError> {
    let random_field = RandomField::new(width, height, mines.clone())?;

    // Copy the random layout into a DefinedField so we can mutate mines freely
    let mut field = DefinedField::new(width, height, mines)?;
    field.set_start_cell(
        random_field.get_start_cell().0,
        random_field.get_start_cell().1,
    );
    let mine_positions: Vec<(u32, u32)> = random_field
        .sorted_fields()
        .filter(|(x, y)| random_field.get_cell(*x, *y) == &crate::Cell::Mine)
        .collect();
    field.initialize(mine_positions);

    let mut failed = FailedMoves::new();
    let mut failed_double = FailedDoubleMoves::new();

    loop {
        let mut solver = create_solver(&field);
        solver.solve();

        if solver.is_solved() {
            debug!("Field solved, no guessing required.");
            return Ok(NoGuessField(field));
        }

        let grid = solver.get_state_grid().clone();
        let frontiers = Frontier::identify_all(&grid, &field);

        if frontiers.is_empty() {
            info!("{}", solver.format_field_state());
            return Err(FieldError::Deadlock(
                "solver stuck with no identifiable frontiers (inaccessible island?)".into(),
            ));
        }

        let revealed_before = solver.revealed_count();

        // --- Single-move attempt ---
        if let Some(candidates) =
            CandidatePicker::pick(&frontiers, &field, &failed, &grid, batch_size)
        {
            let best = candidates
                .into_par_iter()
                .map(|(remove, place)| {
                    let mut clone = field.clone();
                    clone.remove_mine(remove.0, remove.1);
                    clone.place_mine(place.0, place.1);

                    let mut s = create_solver(&clone);
                    s.solve();
                    let count = s.revealed_count();
                    (remove, place, count, clone)
                })
                .max_by_key(|&(_, _, count, _)| count);

            let (remove, place, revealed_after, best_clone) = best.unwrap();

            if revealed_after > revealed_before {
                field = best_clone;
                failed.clear();
                failed_double = FailedDoubleMoves::new();
                debug!(
                    "Kept single relocation {:?} → {:?}: {} → {} revealed",
                    remove, place, revealed_before, revealed_after
                );
            } else {
                failed.insert((remove, place));
                debug!(
                    "Discarded single relocation {:?} → {:?}: no improvement ({} revealed, failed moves: {})",
                    remove,
                    place,
                    revealed_after,
                    failed.len()
                );
            }
            continue;
        }

        // --- Double-move fallback ---
        debug!(
            "Single moves exhausted, trying double relocations (failed_double: {})",
            failed_double.len()
        );

        match CandidatePicker::pick_double(&frontiers, &field, &failed_double, &grid, batch_size) {
            Some(candidates) => {
                let best = candidates
                    .into_par_iter()
                    .map(|(r1, p1, r2, p2)| {
                        let mut clone = field.clone();
                        clone.remove_mine(r1.0, r1.1);
                        clone.place_mine(p1.0, p1.1);
                        clone.remove_mine(r2.0, r2.1);
                        clone.place_mine(p2.0, p2.1);

                        let mut s = create_solver(&clone);
                        s.solve();
                        let count = s.revealed_count();
                        (r1, p1, r2, p2, count, clone)
                    })
                    .max_by_key(|&(_, _, _, _, count, _)| count);

                let (r1, p1, r2, p2, revealed_after, best_clone) = best.unwrap();

                if revealed_after > revealed_before {
                    field = best_clone;
                    failed.clear();
                    failed_double = FailedDoubleMoves::new();
                    debug!(
                        "Kept double relocation ({:?}→{:?}, {:?}→{:?}): {} → {} revealed",
                        r1, p1, r2, p2, revealed_before, revealed_after
                    );
                } else {
                    failed_double.insert((r1, p1, r2, p2));
                    debug!(
                        "Discarded double relocation ({:?}→{:?}, {:?}→{:?}): no improvement ({} revealed, failed doubles: {})",
                        r1,
                        p1,
                        r2,
                        p2,
                        revealed_after,
                        failed_double.len()
                    );
                }
            }
            None => {
                info!("{}", solver.format_field_state());
                return Err(FieldError::Deadlock(
                    "all single and double relocations exhausted — layout cannot be made no-guess"
                        .into(),
                ));
            }
        }
    }
}
