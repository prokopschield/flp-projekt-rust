// =============================================================================
// Integration tests — Dungeon Map Scribe
//
// These tests are provided as part of the project skeleton.
// They are visible, runnable, and cover the main scenarios.
//
// Passing all of these tests does not guarantee full credit — grading
// also uses a separate set of hidden tests covering edge cases.
//
// Run with: cargo test --tests
// =============================================================================

use dungeon_scribe::{reachable_floor_size, DungeonMap, ParseError, Tile, ValidationError};

// ── Helper constants ──────────────────────────────────────────────────────────

const SIMPLE_MAP: &str = include_str!("../maps/simple.txt");

const MINI_VALID: &str = "\
####\n\
#@X#\n\
#..#\n\
####";

// ── Tile parsing (FR-1) ───────────────────────────────────────────────────────

#[test]
fn tile_wall_parses() {
    assert_eq!(Tile::from_char('#'), Ok(Tile::Wall));
}

#[test]
fn tile_player_start_parses() {
    assert_eq!(Tile::from_char('@'), Ok(Tile::PlayerStart));
}

#[test]
fn tile_exit_parses() {
    assert_eq!(Tile::from_char('X'), Ok(Tile::Exit));
}

#[test]
fn tile_unknown_char_returns_error() {
    assert!(Tile::from_char('?').is_err());
}

#[test]
fn tile_space_returns_error() {
    assert!(Tile::from_char(' ').is_err());
}

// ── Map parsing (FR-2) ────────────────────────────────────────────────────────

#[test]
fn parse_empty_input_returns_empty_input_error() {
    assert_eq!(DungeonMap::parse(""), Err(ParseError::EmptyInput));
}

#[test]
fn parse_whitespace_only_returns_empty_input_error() {
    assert_eq!(DungeonMap::parse("   \n\n  "), Err(ParseError::EmptyInput));
}

#[test]
fn parse_simple_map_correct_dimensions() {
    let map = DungeonMap::parse(SIMPLE_MAP).unwrap();
    assert_eq!(map.width(), 10);
    assert_eq!(map.height(), 8);
}

#[test]
fn parse_jagged_map_returns_error() {
    let input = "###\n##\n###";
    assert_eq!(
        DungeonMap::parse(input),
        Err(ParseError::JaggedMap { row: 1, expected: 3, found: 2 })
    );
}

#[test]
fn parse_unknown_tile_carries_position() {
    let input = "###\n#?#\n###";
    assert_eq!(
        DungeonMap::parse(input),
        Err(ParseError::UnknownTile { c: '?', row: 1, col: 1 })
    );
}

// ── Tile counting (FR-3) ──────────────────────────────────────────────────────

#[test]
fn count_tiles_absent_tile_not_in_map() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    let counts = map.count_tiles();
    // MINI_VALID has no Enemy — it must not be in the HashMap, not even with value 0
    assert!(!counts.contains_key(&Tile::Enemy));
}

#[test]
fn count_tiles_wall_count_correct() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    let counts = map.count_tiles();
    assert_eq!(counts.get(&Tile::Wall), Some(&12));
}

// ── Entity search (FR-4) ──────────────────────────────────────────────────────

#[test]
fn find_all_absent_tile_returns_empty_vec() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    assert_eq!(map.find_all(Tile::Enemy), vec![]);
}

#[test]
fn find_all_player_start_correct_position() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    assert_eq!(map.find_all(Tile::PlayerStart), vec![(1, 1)]);
}

#[test]
fn find_all_enemies_sorted_row_major() {
    // simple.txt has enemies at (3,3), (3,7), (5,4)
    let map = DungeonMap::parse(SIMPLE_MAP).unwrap();
    let enemies = map.find_all(Tile::Enemy);
    assert_eq!(enemies, vec![(3, 3), (3, 7), (5, 4)]);
}

// ── Validation (FR-5) ─────────────────────────────────────────────────────────

#[test]
fn validate_valid_map_returns_ok() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    assert!(map.validate().is_ok());
}

#[test]
fn validate_missing_player_start_detected() {
    // Map without PlayerStart
    let map = DungeonMap::parse("####\n#..X\n####").unwrap();
    let errs = map.validate().unwrap_err();
    assert!(errs.contains(&ValidationError::MissingPlayerStart));
}

#[test]
fn validate_no_exit_detected() {
    let map = DungeonMap::parse("####\n#@.#\n#..#\n####").unwrap();
    let errs = map.validate().unwrap_err();
    assert!(errs.contains(&ValidationError::NoExit));
}

#[test]
fn validate_accumulates_multiple_errors() {
    // Map without PlayerStart and without Exit — both errors must be in the result
    let map = DungeonMap::parse("####\n#..#\n#..#\n####").unwrap();
    let errs = map.validate().unwrap_err();
    assert!(errs.contains(&ValidationError::MissingPlayerStart));
    assert!(errs.contains(&ValidationError::NoExit));
}

// ── Reachability (FR-6) ───────────────────────────────────────────────────────

#[test]
fn reachable_wall_start_returns_zero() {
    let map = DungeonMap::parse("###\n#@#\n###").unwrap();
    assert_eq!(reachable_floor_size(&map, (0, 0)), 0);
}

#[test]
fn reachable_out_of_bounds_returns_zero() {
    let map = DungeonMap::parse("###\n#@#\n###").unwrap();
    assert_eq!(reachable_floor_size(&map, (99, 99)), 0);
}

#[test]
fn reachable_isolated_tile_counts_one() {
    // The player is surrounded by walls on all sides
    let map = DungeonMap::parse("###\n#@#\n###").unwrap();
    assert_eq!(reachable_floor_size(&map, (1, 1)), 1);
}

#[test]
fn reachable_simple_corridor() {
    // Corridor with 5 walkable tiles
    let map = DungeonMap::parse("#######\n#@...X#\n#######").unwrap();
    assert_eq!(reachable_floor_size(&map, (1, 1)), 5);
}

#[test]
fn reachable_disconnected_regions_player_only() {
    // A wall in the middle splits the map — the player cannot reach the right side
    let map = DungeonMap::parse("#######\n#@..#.#\n#######").unwrap();
    assert_eq!(reachable_floor_size(&map, (1, 1)), 3);
}

// ── Report generation (FR-7) ──────────────────────────────────────────────────

#[test]
fn report_starts_with_header() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    let report = dungeon_scribe::generate_report(&map);
    assert!(report.starts_with("=== Dungeon Report ===\n"));
}

#[test]
fn report_no_trailing_newline() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    let report = dungeon_scribe::generate_report(&map);
    assert!(!report.ends_with('\n'));
}

#[test]
fn report_contains_correct_dimensions() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    let report = dungeon_scribe::generate_report(&map);
    assert!(report.contains("Dimensions: 4 x 4\n"));
}

#[test]
fn report_valid_map_shows_ok() {
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    assert!(dungeon_scribe::generate_report(&map).contains("Validation: OK\n"));
}

#[test]
fn report_invalid_map_shows_failed() {
    let map = DungeonMap::parse("####\n#..#\n#..#\n####").unwrap();
    let report = dungeon_scribe::generate_report(&map);
    assert!(report.contains("Validation: FAILED\n"));
    assert!(report.contains("  - Missing PlayerStart\n"));
}

#[test]
fn report_absent_tile_not_listed() {
    // MINI_VALID has no Enemy — it must not be listed in the Tiles section
    let map = DungeonMap::parse(MINI_VALID).unwrap();
    assert!(!dungeon_scribe::generate_report(&map).contains("Enemy:"));
}