// ============================================================
// dungeon-scribe — report generation
//
// You must implement the function body below.
// ============================================================

use std::fmt::Write;

use crate::model::DungeonMap;
use crate::reachable_floor_size;
use crate::tile::Tile::{Door, Enemy, Exit, Floor, PlayerStart, Trap, Treasure, Wall};

/// Generates a human-readable summary report for the given map.
///
/// # Required output format
///
/// ```text
/// === Dungeon Report ===
/// Dimensions: 10 x 8
/// Tiles:
///   Wall:          26
///   Floor:         38
///   PlayerStart:    1
///   Enemy:          3
///   Treasure:       2
///   Exit:           1
/// Validation: OK
/// Reachable floor from player: 42
/// ```
///
/// If validation fails:
///
/// ```text
/// Validation: FAILED
///   - Missing PlayerStart
///   - No Exit found
/// ```
///
/// # Rules
///
/// - **Dimensions** — `width x height` (columns × rows).
/// - **Tiles section** — list only tile types present in the map (count ≥ 1),
///   in this canonical order:
///   `Wall`, `Floor`, `PlayerStart`, `Enemy`, `Treasure`, `Exit`, `Door`, `Trap`.
/// - **Tile count alignment** — right-align the count in a field of width 4.
///   Use the format string `{:>4}` (see the Tips section of the assignment).
/// - **Validation** — call `self.validate()`. Print `OK` on success, or
///   `FAILED` followed by one line per error (two-space indent, dash prefix).
/// - **Reachable floor** — call `reachable_floor_size` with the `PlayerStart`
///   position as `start`. If there is no `PlayerStart`, write `0`.
/// - **Line endings** — every line ends with exactly one `\n`. There is no
///   trailing blank line after the last line.
///
/// # Hint
///
/// `format!()` supports alignment: `format!("  {:14}{:>4}", label, count)`.
/// The tile label widths in the example above are not a coincidence —
/// `"PlayerStart:"` is the longest label (12 chars + `:`). Pad all labels
/// to the same width for alignment.
#[must_use]
pub fn generate_report(map: &DungeonMap) -> String {
    let mut a = "=== Dungeon Report ===\n".to_string();

    writeln!(a, "Dimensions: {} x {}", map.width(), map.height()).unwrap_or_default();
    writeln!(a, "Tiles:").unwrap_or_default();

    let count = map.count_tiles();

    for tile in [Wall, Floor, PlayerStart, Enemy, Treasure, Exit, Door, Trap] {
        if let Some(&count) = count.get(&tile)
            && count > 0
        {
            // format with colon
            let tile = format!("{tile}:");

            writeln!(a, "  {tile:14}{count:>4}").unwrap_or_default();
        }
    }

    if let Err(err) = map.validate() {
        writeln!(a, "Validation: FAILED").unwrap_or_default();

        for err in err {
            writeln!(a, "  - {err}").unwrap_or_default();
        }
    } else {
        writeln!(a, "Validation: OK").unwrap_or_default();
    }

    write!(
        a,
        "Reachable floor from player: {}",
        map.find_player_start()
            .map_or(0, |position| reachable_floor_size(map, position))
    )
    .unwrap_or_default();

    // using .unwrap_or_default() everywhere because .expect() is forbidden

    a
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::model::DungeonMap;

    // Add your own tests here.

    #[test]
    fn report_starts_with_header() {
        let map = DungeonMap::parse("##\n#@\n##").unwrap();
        let report = generate_report(&map);
        assert!(report.starts_with("=== Dungeon Report ===\n"));
    }
}
