// Create the NG Minesweeper Field

/*
if not solvable, check if there are multiple islands
    if yes, take a random mine from an island border and move it to another island border
        then try solving again? possible from the current sovler state theoretically
    if no, check minecount, unrevealed etc if it makes sense to just move the mine somewhere else (from the border away)
*/