// ============================================================
// dungeon-scribe — library root
//
// PROVIDED — do not modify this file.
// ============================================================

pub mod analysis;
pub mod error;
pub mod model;
pub mod report;
pub mod tile;

pub use analysis::reachable_floor_size;
pub use error::{ParseError, ValidationError};
pub use model::{DungeonMap, Position};
pub use report::generate_report;
pub use tile::Tile;
