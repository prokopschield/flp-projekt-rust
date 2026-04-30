// ============================================================
// dungeon-scribe — reachability analysis
//
// You must implement the function body below.
// You may add private helper functions.
// ============================================================

mod bitmap;
mod bitvec;

pub use bitmap::{BitMap, BitMapError};
pub use bitvec::{BitVec, BitVecError};

use crate::model::{DungeonMap, Position};

/// Returns the number of passable tiles reachable from `start` via
/// 4-directional movement (up, down, left, right), without crossing walls.
///
/// A tile is passable if `Tile::is_passable()` returns `true` for it.
/// The starting tile itself is counted if it is passable.
///
/// # Returns
///
/// - `0` if `start` is out of bounds.
/// - `0` if the tile at `start` is not passable (i.e. is a `Wall`).
/// - Otherwise, the total number of passable tiles in the connected region
///   that contains `start`, including `start` itself.
///
/// # Implementation notes
///
/// A standard BFS or DFS is the expected approach:
/// - Use a `VecDeque<Position>` (BFS) or `Vec<Position>` (DFS) as the frontier.
/// - Use a `HashSet<Position>` to track visited positions.
/// - `Position` is `(usize, usize)` which is `Copy`, so you can push positions
///   into the queue without lifetime issues.
///
/// # Warning — usize subtraction
///
/// `Position` uses `usize`. Subtracting from `0_usize` causes a panic.
/// When generating the four neighbours of a position, guard against
/// `row == 0` and `col == 0` **before** subtracting.
#[must_use]
pub fn reachable_floor_size(map: &DungeonMap, start: Position) -> usize {
    if let Some(tile) = map.get(start)
        && tile.is_passable()
    {
        // Great! Let's go check it out :-)
    } else {
        return 0; // not passable
    }

    let directions = [
        |row: usize, col: usize| (row.checked_sub(1), Some(col)),
        |row: usize, col: usize| (row.checked_add(1), Some(col)),
        |row: usize, col: usize| (Some(row), col.checked_sub(1)),
        |row: usize, col: usize| (Some(row), col.checked_add(1)),
    ];

    let width = map.width();
    let height = map.height();

    let mut queue = vec![start];
    let mut visited = BitMap::new(width, height);

    // count passable tiles
    let mut count = 0;

    while let Some((row, col)) = queue.pop() {
        // ignore already-visited tiles
        if matches!(visited.get(row, col), Ok(false)) {
            visited.set(row, col, true).unwrap_or_default();

            count += 1;

            for step in directions {
                let (Some(new_row), Some(new_col)) = step(row, col) else {
                    // integer underflow
                    continue;
                };

                if let Some(tile) = map.get((new_row, new_col))
                    && tile.is_passable()
                    && matches!(visited.get(new_row, new_col), Ok(false))
                {
                    queue.push((new_row, new_col));
                }
            }
        }
    }

    count
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DungeonMap;

    // Add your own tests here if needed.

    #[test]
    fn wall_start_returns_zero() {
        let map = DungeonMap::parse("###\n#@#\n###").unwrap();
        // (0, 0) is a Wall
        assert_eq!(reachable_floor_size(&map, (0, 0)), 0);
    }

    #[test]
    fn out_of_bounds_start_returns_zero() {
        let map = DungeonMap::parse("###\n#@#\n###").unwrap();
        assert_eq!(reachable_floor_size(&map, (99, 99)), 0);
    }
}
