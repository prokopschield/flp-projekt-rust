// ============================================================
// dungeon-scribe — report generation
//
// You must implement the function body below.
// ============================================================

use crate::model::DungeonMap;

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
pub fn generate_report(map: &DungeonMap) -> String {
    todo!()
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
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
