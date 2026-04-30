//! Additional integration tests covering edge cases not in `integration_tests.rs`.
#![allow(clippy::expect_used)]

use dungeon_scribe::{
    DungeonMap, ParseError, ValidationError, generate_report, reachable_floor_size,
};

// --- Reachability ---

#[test]
fn reachable_non_square_map() {
    // 5 wide × 3 tall; catches any latent row/col swap.
    let map = DungeonMap::parse("#####\n#@..#\n#####").expect("valid map");

    assert_eq!(reachable_floor_size(&map, (1, 1)), 3);
}

#[test]
fn reachable_tall_non_square_map() {
    // 3 wide × 5 tall; same idea, opposite aspect ratio.
    let map = DungeonMap::parse("###\n#@#\n#.#\n#.#\n###").expect("valid map");

    assert_eq!(reachable_floor_size(&map, (1, 1)), 3);
}

#[test]
fn reachable_diagonal_not_crossable() {
    // Two passable cells touching only diagonally; must not be reached.
    // Start at (1, 1); (2, 2) is diagonally adjacent but separated by walls.
    let map = DungeonMap::parse("###\n#@#\n##.").expect("valid map");

    assert_eq!(reachable_floor_size(&map, (1, 1)), 1);
}

#[test]
fn reachable_open_room_counts_all_floor() {
    // 5×3 interior = 15 passable tiles, surrounded by walls.
    let map = DungeonMap::parse("#######\n#@....#\n#.....#\n#.....#\n#######").expect("valid map");

    assert_eq!(reachable_floor_size(&map, (1, 1)), 15);
}

#[test]
fn reachable_expands_in_all_four_directions() {
    // Plus-shaped corridor: start at center, paths in all four directions.
    let map = DungeonMap::parse("##.##\n##.##\n.@..X\n##.##\n##.##").expect("valid map");

    assert_eq!(reachable_floor_size(&map, (2, 1)), 9);
}

#[test]
fn reachable_includes_all_passable_tile_kinds() {
    // Every non-Wall variant is passable per Tile::is_passable.
    // Row 1 contains @ . E T + ^ X; all seven must be reached.
    let map = DungeonMap::parse("#########\n#@.ET+^X#\n#########").expect("valid map");

    assert_eq!(reachable_floor_size(&map, (1, 1)), 7);
}

// --- Validation ---

#[test]
fn validate_multiple_player_starts_detected() {
    let map = DungeonMap::parse("####\n#@@#\n#.X#\n####").expect("valid map");
    let errs = map.validate().expect_err("two @ tiles");

    let multi = errs
        .iter()
        .find(|e| matches!(e, ValidationError::MultiplePlayerStarts(_)))
        .expect("MultiplePlayerStarts should be present");

    if let ValidationError::MultiplePlayerStarts(positions) = multi {
        assert_eq!(positions, &vec![(1, 1), (1, 2)]);
    }
}

#[test]
fn validate_no_floor_detected() {
    // Valid PlayerStart and Exit, but no Floor tile at all.
    let map = DungeonMap::parse("####\n#@X#\n####").expect("valid map");
    let errs = map.validate().expect_err("no Floor");

    assert!(errs.contains(&ValidationError::NoFloor));
}

#[test]
fn validate_error_order_is_canonical() {
    // Triggers all three error categories. Spec-mandated order:
    //   PlayerStart-related, then NoExit, then NoFloor.
    // This map has no PlayerStart, no Exit, and no Floor.
    let map = DungeonMap::parse("####\n####\n####").expect("valid map");
    let errs = map.validate().expect_err("multiple errors");

    assert_eq!(
        errs,
        vec![
            ValidationError::MissingPlayerStart,
            ValidationError::NoExit,
            ValidationError::NoFloor,
        ]
    );
}

// --- Report ---

#[test]
fn report_reachable_zero_when_no_player_start() {
    let map = DungeonMap::parse("####\n#.X#\n####").expect("valid map");
    let report = generate_report(&map);

    assert!(report.contains("Reachable floor from player: 0"));
}

#[test]
fn report_tile_line_has_correct_alignment() {
    // MINI-style 4×4 map with 12 walls. Verifies the colon, padding, and
    // right-aligned count column all line up exactly.
    let map = DungeonMap::parse("####\n#@X#\n#..#\n####").expect("valid map");
    let report = generate_report(&map);

    assert!(
        report.contains("  Wall:           12\n"),
        "expected exact-aligned Wall row, got:\n{report}"
    );
}

#[test]
fn report_tile_section_in_canonical_order() {
    // Every tile variant appears exactly once. Their labels must appear in
    // canonical order (Wall, Floor, PlayerStart, Enemy, Treasure, Exit, Door, Trap).
    let map = DungeonMap::parse("##########\n#@.ET+^X.#\n##########").expect("valid map");
    let report = generate_report(&map);

    let expected_order = [
        "Wall:",
        "Floor:",
        "PlayerStart:",
        "Enemy:",
        "Treasure:",
        "Exit:",
        "Door:",
        "Trap:",
    ];

    let mut last = 0;

    for label in expected_order {
        let idx = report
            .find(label)
            .unwrap_or_else(|| panic!("label {label} missing from report:\n{report}"));

        assert!(
            idx >= last,
            "label {label} appears out of order in:\n{report}"
        );

        last = idx;
    }
}

#[test]
fn report_last_line_is_reachable_floor() {
    let map = DungeonMap::parse("####\n#@X#\n#..#\n####").expect("valid map");
    let report = generate_report(&map);

    let last_line = report.lines().last().expect("report is non-empty");

    assert!(
        last_line.starts_with("Reachable floor from player: "),
        "expected last line to be the reachable-floor line, got: {last_line:?}"
    );
}

// --- Parsing edge cases ---

#[test]
fn parse_windows_line_endings() {
    let input = "####\r\n#@X#\r\n#..#\r\n####";
    let map = DungeonMap::parse(input).expect("\\r\\n endings must be accepted");

    assert_eq!(map.width(), 4);
    assert_eq!(map.height(), 4);
}

#[test]
fn parse_single_row_map() {
    let map = DungeonMap::parse("#@.X#").expect("valid 1-row map");

    assert_eq!(map.width(), 5);
    assert_eq!(map.height(), 1);
}

#[test]
fn parse_single_column_map() {
    let map = DungeonMap::parse("#\n@\n.\nX\n#").expect("valid 1-column map");

    assert_eq!(map.width(), 1);
    assert_eq!(map.height(), 5);
}

#[test]
fn parse_unknown_tile_position_non_diagonal() {
    // The error must report the actual (row, col), not a swapped pair.
    // Place the bad char at (2, 4) so a swap would be detectable.
    let input = "######\n######\n####?#";

    assert_eq!(
        DungeonMap::parse(input),
        Err(ParseError::UnknownTile {
            c: '?',
            row: 2,
            col: 4
        })
    );
}
