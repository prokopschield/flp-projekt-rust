// ============================================================
// dungeon-scribe — error types
//
// PROVIDED — do not modify the variants or Display implementations.
// You may read this file to understand what errors your functions
// must produce.
// ============================================================

use std::fmt;

use crate::model::Position;

// ── Parse errors ─────────────────────────────────────────────────────────────

/// Errors that can occur while parsing a dungeon map string.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// The input string was empty or contained only whitespace/newlines.
    EmptyInput,

    /// A row had a different length from the first row.
    ///
    /// `row` is the 0-based index of the offending row.
    /// `expected` is the width of row 0.
    /// `found` is the actual width of the offending row.
    JaggedMap {
        row: usize,
        expected: usize,
        found: usize,
    },

    /// An unrecognised character was encountered.
    ///
    /// `c` is the character.
    /// `row` and `col` are its 0-based position in the input.
    UnknownTile { c: char, row: usize, col: usize },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => {
                write!(f, "map input is empty")
            }
            ParseError::JaggedMap { row, expected, found } => {
                write!(
                    f,
                    "row {} has width {} but expected {} (from row 0)",
                    row, found, expected
                )
            }
            ParseError::UnknownTile { c, row, col } => {
                write!(
                    f,
                    "unknown tile character {:?} at row {}, col {}",
                    c, row, col
                )
            }
        }
    }
}

// ── Validation errors ─────────────────────────────────────────────────────────

/// Errors that can be detected when validating a successfully-parsed map.
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    /// No `PlayerStart` tile was found. Exactly one is required.
    MissingPlayerStart,

    /// More than one `PlayerStart` tile was found.
    ///
    /// The `Vec` contains all of their positions, sorted row-first.
    MultiplePlayerStarts(Vec<Position>),

    /// No `Exit` tile was found. At least one is required.
    NoExit,

    /// No `Floor` tile was found.
    NoFloor,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::MissingPlayerStart => {
                write!(f, "Missing PlayerStart")
            }
            ValidationError::MultiplePlayerStarts(positions) => {
                write!(f, "Multiple PlayerStart tiles at {:?}", positions)
            }
            ValidationError::NoExit => {
                write!(f, "No Exit found")
            }
            ValidationError::NoFloor => {
                write!(f, "No Floor tiles found")
            }
        }
    }
}
