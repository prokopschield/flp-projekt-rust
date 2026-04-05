// ============================================================
// dungeon-scribe — tile type
//
// The enum variants are provided. You must implement the two
// methods below.
// ============================================================

use crate::error::ParseError;

/// A single tile in a dungeon map.
///
/// The character-to-variant mapping is:
///
/// | Char | Variant      |
/// |------|--------------|
/// | `#`  | `Wall`       |
/// | `.`  | `Floor`      |
/// | `@`  | `PlayerStart`|
/// | `E`  | `Enemy`      |
/// | `T`  | `Treasure`   |
/// | `X`  | `Exit`       |
/// | `+`  | `Door`       |
/// | `^`  | `Trap`       |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Wall,
    Floor,
    PlayerStart,
    Enemy,
    Treasure,
    Exit,
    Door,
    Trap,
}

impl Tile {
    /// Parse a single character into the corresponding `Tile` variant.
    ///
    /// # Errors
    ///
    /// Returns `Err(ParseError::UnknownTile { c, row: 0, col: 0 })` for any
    /// character not listed in the table above.
    ///
    /// Note: the `row` and `col` fields are set to `0` here because this
    /// method does not know the position. `DungeonMap::parse` is responsible
    /// for injecting the correct position into the error before returning it.
    pub fn from_char(c: char) -> Result<Tile, ParseError> {
        todo!()
    }

    /// Returns `true` if this tile type allows movement through it.
    ///
    /// Every tile except `Wall` is passable. This method is used by the
    /// flood-fill algorithm in `analysis::reachable_floor_size`.
    #[must_use]
    pub const fn is_passable(&self) -> bool {
        !matches!(self, Self::Wall)
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Add your own tests here.

    #[test]
    fn wall_char_parses_to_wall() {
        assert_eq!(Tile::from_char('#'), Ok(Tile::Wall));
    }

    #[test]
    fn unknown_char_returns_error() {
        assert!(Tile::from_char('?').is_err());
    }
}
