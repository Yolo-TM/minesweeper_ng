use std::collections::HashSet;

pub(super) type FailedMoves = HashSet<((u32, u32), (u32, u32))>;
pub(super) type FailedDoubleMoves = HashSet<((u32, u32), (u32, u32), (u32, u32), (u32, u32))>;
