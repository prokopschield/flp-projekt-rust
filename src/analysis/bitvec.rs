// Bit Vector implementation
// made for this project, so probably not optimal

const BITS_PER_ITEM: usize = std::mem::size_of::<usize>();

pub struct BitVec {
    items: Vec<usize>,
}

impl BitVec {
    #[must_use]
    pub fn new(size: usize) -> Self {
        let size = size.div_ceil(BITS_PER_ITEM);

        let items = vec![0; size];

        Self { items }
    }

    /// Gets a bit from this `BitVec`
    ///
    /// # Errors
    ///
    /// Returns [`BitVecOpError::OutOfBounds`] if index is out of bounds.
    pub fn get(&self, index: usize) -> BitVecResult<bool> {
        let (quotient, remainder) = (index / BITS_PER_ITEM, index % BITS_PER_ITEM);
        let item = self
            .items
            .get(quotient)
            .ok_or_else(|| BitVecError::out_of_bounds(index, quotient))?;

        Ok((item >> remainder) & 1 != 0)
    }

    /// Sets a bit in this `BitVec`
    ///
    /// # Errors
    ///
    /// Returns [`BitVecOpError::OutOfBounds`] if index is out of bounds.
    pub fn set(&mut self, index: usize, value: bool) -> BitVecResult {
        let (quotient, remainder) = (index / BITS_PER_ITEM, index % BITS_PER_ITEM);
        let mask = 1 << remainder;

        let item = self
            .items
            .get_mut(quotient)
            .ok_or_else(|| BitVecError::out_of_bounds(index, quotient))?;

        if value {
            *item |= mask;
        } else {
            *item &= !mask;
        }

        Ok(())
    }

    /// Returns the logical size of this [`BitVec`] in bits.
    #[must_use]
    pub fn size(&self) -> usize {
        self.items.len().saturating_mul(BITS_PER_ITEM)
    }
}

pub type BitVecResult<T = ()> = Result<T, BitVecError>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitVecError {
    OutOfBounds { index: usize, size: usize },
}

impl BitVecError {
    #[must_use]
    pub const fn out_of_bounds(index: usize, quotient: usize) -> Self {
        Self::OutOfBounds {
            index,
            size: quotient * BITS_PER_ITEM,
        }
    }
}

impl std::fmt::Display for BitVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds { index, size } => {
                write!(f, "Index out of bounds: {index} >= {size}")
            }
        }
    }
}

impl std::error::Error for BitVecError {}
