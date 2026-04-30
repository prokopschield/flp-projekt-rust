use std::{error::Error, fmt::Display};

use super::{BitVec, BitVecError};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitMap {
    bits: BitVec,
    width: usize,
    height: usize,
}

impl BitMap {
    /// Allocates a new `width`×`height` [`BitMap`].
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let size = width.saturating_mul(height);
        let bits = BitVec::new(size);

        Self {
            bits,
            width,
            height,
        }
    }

    /// Returns the width of this [`BitMap`].
    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this [`BitMap`].
    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }

    /// Returns the bit at a given row and column.
    ///
    /// # Errors
    ///
    /// - [`BitMapError::RowOutOfBounds`] is returned if `row` is out of bounds.
    /// - [`BitMapError::ColumnOutOfBounds`] is returned if `column` is out of bounds.
    /// - [`BitMapError::BitVecError`] should be unreachable.
    pub fn get(&self, row: usize, column: usize) -> BitMapResult<bool> {
        let index = self.calculate_offset(row, column)?;
        let value = self.bits.get(index)?;

        Ok(value)
    }

    /// Sets the bit at a given row and column.
    ///
    /// # Errors
    ///
    /// - [`BitMapError::RowOutOfBounds`] is returned if `row` is out of bounds.
    /// - [`BitMapError::ColumnOutOfBounds`] is returned if `column` is out of bounds.
    /// - [`BitMapError::BitVecError`] should be unreachable.
    pub fn set(&mut self, row: usize, column: usize, value: bool) -> BitMapResult {
        let index = self.calculate_offset(row, column)?;

        self.bits.set(index, value).map_err(Into::into)
    }

    /// Calculates the bitvec offset at given coordinates.
    ///
    /// # Errors
    ///
    /// - [`BitMapError::RowOutOfBounds`] is returned if `row` is out of bounds.
    /// - [`BitMapError::ColumnOutOfBounds`] is returned if `column` is out of bounds.
    const fn calculate_offset(&self, row: usize, column: usize) -> BitMapResult<usize> {
        if column >= self.width {
            return Err(BitMapError::ColumnOutOfBounds {
                column,
                width: self.width,
            });
        }

        if row >= self.height {
            return Err(BitMapError::RowOutOfBounds {
                row,
                height: self.height,
            });
        }

        Ok(self.width.saturating_mul(row).saturating_add(column))
    }
}

pub type BitMapResult<T = ()> = Result<T, BitMapError>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitMapError {
    BitVecError(BitVecError),
    RowOutOfBounds { row: usize, height: usize },
    ColumnOutOfBounds { column: usize, width: usize },
}

impl Display for BitMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BitVecError(err) => err.fmt(f),
            Self::RowOutOfBounds { row, height } => {
                write!(f, "Row out of bounds: {row} >= {height}")
            }
            Self::ColumnOutOfBounds { column, width } => {
                write!(f, "Column out of bounds: {column} >= {width}")
            }
        }
    }
}

impl Error for BitMapError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BitVecError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<BitVecError> for BitMapError {
    fn from(err: BitVecError) -> Self {
        Self::BitVecError(err)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::BitMap;

    const PATTERN_SIZE: usize = 1024;

    #[test]
    fn pattern() {
        let mut bitmap = BitMap::new(PATTERN_SIZE, PATTERN_SIZE);

        let value_at = |row: usize, col: usize| (row ^ col).count_zeros().is_multiple_of(2);

        for i in 0..PATTERN_SIZE {
            for j in 0..PATTERN_SIZE {
                bitmap
                    .set(i, j, value_at(i, j))
                    .expect("Setting a value within bounds shouldn't fail");
            }
        }

        for i in 0..PATTERN_SIZE {
            for j in 0..PATTERN_SIZE {
                let value = bitmap
                    .get(i, j)
                    .expect("Reading a value within bounds shouldn't fail");

                assert_eq!(
                    value,
                    value_at(i, j),
                    "Value should match what was set earlier."
                );
            }
        }
    }

    #[test]
    fn bounds() {
        let bitmap = BitMap::new(PATTERN_SIZE, PATTERN_SIZE);

        assert_eq!(
            bitmap.get(PATTERN_SIZE, PATTERN_SIZE - 1),
            Err(crate::analysis::BitMapError::RowOutOfBounds {
                row: PATTERN_SIZE,
                height: PATTERN_SIZE
            })
        );

        assert_eq!(
            bitmap.get(PATTERN_SIZE - 1, PATTERN_SIZE),
            Err(crate::analysis::BitMapError::ColumnOutOfBounds {
                column: PATTERN_SIZE,
                width: PATTERN_SIZE
            })
        );
    }
}
