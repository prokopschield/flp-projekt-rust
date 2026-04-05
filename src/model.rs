// ============================================================
// dungeon-scribe — DungeonMap and Position
//
// The struct definition and all method signatures are provided.
// You must implement every method body that contains `todo!()`.
//
// You may add private helper methods freely.
// Do not change any public signature.
// ============================================================

use std::collections::HashMap;

use crate::error::{ParseError, ValidationError};
use crate::tile::Tile;

/// A position in the dungeon map, represented as `(row, col)`.
///
/// Both `row` and `col` are 0-based.
pub type Position = (usize, usize);

/// A successfully parsed dungeon map.
///
/// The grid is stored as a `Vec` of rows, each row being a `Vec<Tile>`.
/// `width` is the number of columns; `height` is the number of rows.
#[derive(Debug, PartialEq)]
pub struct DungeonMap {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl DungeonMap {
    // ── Parsing ──────────────────────────────────────────────────────────────

    /// Parse a multi-line `&str` into a `DungeonMap`.
    ///
    /// Each line of the input corresponds to one row of the map.
    /// Lines are split on `'\n'`; any trailing `'\r'` is stripped first so
    /// that both Unix (`\n`) and Windows (`\r\n`) files are accepted.
    ///
    /// # Errors
    ///
    /// - `ParseError::EmptyInput` — input has no non-empty lines.
    ///   Lines that contain only whitespace also count as empty.
    /// - `ParseError::JaggedMap` — a row has a different width from row 0.
    ///   The error carries the 0-based index of the offending row, the
    ///   expected width (from row 0), and the actual width found.
    /// - `ParseError::UnknownTile` — a character is not recognised.
    ///   The error carries the character and its `(row, col)` position.
    ///
    /// # Hint
    ///
    /// The idiomatic Rust approach is to use `lines().enumerate()` and then
    /// `chars().enumerate()` with `map` and `collect::<Result<Vec<_>, _>>()`.
    /// This pattern propagates errors naturally while building the
    /// `Vec<Vec<Tile>>`.
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tiles = input
            .split('\n')
            .map(str::trim_end)
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match Tile::from_char(c) {
                        Ok(tile) => Ok(tile),
                        Err(ParseError::UnknownTile { c, .. }) => {
                            Err(ParseError::UnknownTile { c, row, col })
                        }
                        Err(other_err) => {
                            // This case is impossible, but I'm not allowed to change the signature of `Tile::from_char`.
                            Err(other_err)
                        }
                    })
                    .collect::<Result<Vec<Tile>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let height = tiles.len();
        let width = tiles.first().ok_or(ParseError::EmptyInput)?.len();

        for (idx, row) in tiles.iter().enumerate() {
            if row.len() != width {
                return Err(ParseError::JaggedMap {
                    row: idx,
                    expected: width,
                    found: row.len(),
                });
            }
        }

        let map = Self {
            tiles,
            width,
            height,
        };

        Ok(map)
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    /// Returns a reference to the tile at `pos = (row, col)`.
    ///
    /// Returns `None` if the position is out of bounds.
    /// The returned reference has the same lifetime as `&self`.
    #[must_use]
    pub fn get(&self, pos: Position) -> Option<&Tile> {
        let (x, y) = pos;

        self.tiles.get(x)?.get(y)
    }

    /// Returns the width of the map (number of columns).
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the map (number of rows).
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an iterator over the rows of the map.
    ///
    /// Each item is a reference to one row (`&Vec<Tile>`).
    ///
    /// Note: returning `impl Iterator` from a method requires understanding
    /// that the iterator borrows from `&self`.
    pub fn rows(&self) -> impl Iterator<Item = &Vec<Tile>> {
        self.tiles.iter()
    }

    // ── Analysis ──────────────────────────────────────────────────────────────

    /// Returns a `HashMap` mapping each `Tile` variant to its count.
    ///
    /// Only tile types that appear at least once are included as keys.
    /// Absent tile types must **not** appear with a count of `0`.
    ///
    /// # Hint
    ///
    /// `self.rows().flat_map(|row| row.iter())` gives you a flat iterator
    /// over every tile in the map.
    #[must_use]
    pub fn count_tiles(&self) -> HashMap<Tile, usize> {
        self.rows()
            .flat_map(|row| row.iter())
            .fold(HashMap::new(), |mut a, tile| {
                a.insert(*tile, a.get(tile).copied().unwrap_or_default() + 1);
                a
            })
    }

    /// Returns all positions where `tile` appears, sorted row-first then
    /// column (i.e. in reading order).
    ///
    /// Returns an empty `Vec` if the tile does not appear in the map.
    #[must_use]
    pub fn find_all(&self, tile: Tile) -> Vec<Position> {
        self.rows()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, t)| tile.eq(t).then_some((x, y)))
            })
            .collect()
    }

    // ── Validation ────────────────────────────────────────────────────────────

    /// Validates the structural integrity of the map.
    ///
    /// Returns `Ok(())` if and only if all of the following hold:
    /// - Exactly one `PlayerStart` tile exists.
    /// - At least one `Exit` tile exists.
    /// - At least one `Floor` tile exists.
    ///
    /// # Errors
    ///
    /// Returns [`Vec<ValidationError>`] if any of the three above conditions are violated.
    ///
    /// The resulting [`Vec`] contains **all** applicable [`ValidationError`]s in the following order:
    /// - [`ValidationError::MissingPlayerStart`] if no [`Tile::PlayerStart`] is present
    /// - [`ValidationError::MultiplePlayerStarts`] if multiple [`Tile::PlayerStart`] are present
    /// - [`ValidationError::NoExit`] if no [`Tile::Exit`] is present
    /// - [`ValidationError::NoFloor`] if no [`Tile::Floor`] is present
    ///
    /// # Important
    ///
    /// Unlike most `Result`-returning functions, this one must **accumulate
    /// all errors** before returning. Do not use `?` for early return here.
    /// Build a `Vec<ValidationError>`, push to it for each failing condition,
    /// and return `Err(errors)` at the end if it is non-empty.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        let tile_count = self.count_tiles();

        match tile_count.get(&Tile::PlayerStart) {
            None | Some(0) => errors.push(ValidationError::MissingPlayerStart),
            Some(1) => {}
            Some(2..) => errors.push(ValidationError::MultiplePlayerStarts(
                self.find_all(Tile::PlayerStart),
            )),
        }

        if !tile_count.contains_key(&Tile::Exit) {
            errors.push(ValidationError::NoExit);
        }

        if !tile_count.contains_key(&Tile::Floor) {
            errors.push(ValidationError::NoFloor);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Add your own tests here.

    #[test]
    fn parse_empty_input_returns_error() {
        assert!(DungeonMap::parse("").is_err());
    }

    #[test]
    fn parse_single_row_map() {
        let map = DungeonMap::parse("###").unwrap();
        assert_eq!(map.width(), 3);
        assert_eq!(map.height(), 1);
    }
}
