// ============================================================
// dungeon-scribe — tile type
//
// The enum variants are provided. You must implement the two
// methods below.
// ============================================================

use std::fmt::Display;

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
    pub const fn from_char(c: char) -> Result<Self, ParseError> {
        match c {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Floor),
            '@' => Ok(Self::PlayerStart),
            'E' => Ok(Self::Enemy),
            'T' => Ok(Self::Treasure),
            'X' => Ok(Self::Exit),
            '+' => Ok(Self::Door),
            '^' => Ok(Self::Trap),
            c => Err(ParseError::UnknownTile { c, col: 0, row: 0 }),
        }
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

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tile::{Door, Enemy, Exit, Floor, PlayerStart, Trap, Treasure, Wall};

        match self {
            Wall => f.pad("Wall"),
            Floor => f.pad("Floor"),
            PlayerStart => f.pad("PlayerStart"),
            Enemy => f.pad("Enemy"),
            Treasure => f.pad("Treasure"),
            Exit => f.pad("Exit"),
            Door => f.pad("Door"),
            Trap => f.pad("Trap"),
        }
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
