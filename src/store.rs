
use std::cmp::PartialEq;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

///
/// This trait represents an abstraction over storage that contains indexable
/// bits.
/// 
pub trait BitStore:
    BitAnd<Self, Output = Self> +
    BitAndAssign<Self> +
    BitOr<Self, Output = Self> +
    BitOrAssign<Self> +
    BitXor<Self, Output = Self> +
    BitXorAssign<Self> +
    Not<Output = Self> +
    PartialEq<Self> +
    Copy +
    Clone +
    Sized {

    ///
    /// A const containing the total number of addressable bits in this type.
    /// 
    const BIT_COUNT: usize;
    
    ///
    /// A const containing the 0 (no bits set) value for this type.
    /// 
    const ZERO: Self;

    ///
    /// A const containing the max (all bits set) value for this type.
    /// 
    const MAX: Self;
    
    ///
    /// Creates a mask used to index a single bit in a value of this type.
    /// Implementations can assume that bit_index < Self::BIT_COUNT.
    /// 
    fn create_bit_mask(bit_index: usize) -> Self;
    
    ///
    /// Creates a mask used to index a range of bits in a value of this type.
    /// Implementations can assume that start_bit < Self::BIT_COUNT and
    /// (start_bit + bit_count) <= Self::BIT_COUNT.
    /// 
    fn create_range_mask(start_bit: usize, bit_count: usize) -> Self;
    
    ///
    /// Counts the number of trailing zeros in a value of this type.
    /// 
    fn trailing_zeros(self) -> usize;

}

impl BitStore for bool {

    const BIT_COUNT: usize = 1;
    const ZERO: Self = false;
    const MAX: Self = true;

    fn create_bit_mask(_bit_index: usize) -> Self {
        true
    }

    fn create_range_mask(_start_bit: usize, bit_count: usize) -> Self {
        bit_count != 0
    }

    fn trailing_zeros(self) -> usize {
        if self {
            0

        } else {
            1
        }
    }
    
}

impl BitStore for u8 {

    const BIT_COUNT: usize = Self::BITS as usize;
    const ZERO: Self = 0;
    const MAX: Self = Self::MAX;

    fn create_bit_mask(bit_index: usize) -> Self {
        1 << bit_index
    }

    fn create_range_mask(start_bit: usize, bit_count: usize) -> Self {
        if bit_count == Self::BIT_COUNT {
            Self::MAX

        } else {
            ((1 << bit_count) - 1) << start_bit
        }
    }

    fn trailing_zeros(self) -> usize {
        Self::trailing_zeros(self) as usize
    }

}

impl BitStore for u16 {

    const BIT_COUNT: usize = Self::BITS as usize;
    const ZERO: Self = 0;
    const MAX: Self = Self::MAX;

    fn create_bit_mask(bit_index: usize) -> Self {
        1 << bit_index
    }

    fn create_range_mask(start_bit: usize, bit_count: usize) -> Self {
        if bit_count == Self::BIT_COUNT {
            Self::MAX

        } else {
            ((1 << bit_count) - 1) << start_bit
        }
    }

    fn trailing_zeros(self) -> usize {
        Self::trailing_zeros(self) as usize
    }

}

impl BitStore for u32 {

    const BIT_COUNT: usize = u32::BITS as usize;
    const ZERO: Self = 0;
    const MAX: Self = u32::MAX;

    fn create_bit_mask(bit_index: usize) -> Self {
        1 << bit_index
    }

    fn create_range_mask(start_bit: usize, bit_count: usize) -> Self {
        if bit_count == Self::BIT_COUNT {
            Self::MAX

        } else {
            ((1 << bit_count) - 1) << start_bit
        }
    }

    fn trailing_zeros(self) -> usize {
        Self::trailing_zeros(self) as usize
    }

}

impl BitStore for u64 {

    const BIT_COUNT: usize = Self::BITS as usize;
    const ZERO: Self = 0;
    const MAX: Self = Self::MAX;

    fn create_bit_mask(bit_index: usize) -> Self {
        1 << bit_index
    }

    fn create_range_mask(start_bit: usize, bit_count: usize) -> Self {
        if bit_count == Self::BIT_COUNT {
            Self::MAX

        } else {
            ((1 << bit_count) - 1) << start_bit
        }
    }

    fn trailing_zeros(self) -> usize {
        Self::trailing_zeros(self) as usize
    }

}

impl BitStore for u128 {

    const BIT_COUNT: usize = Self::BITS as usize;
    const ZERO: Self = 0;
    const MAX: Self = Self::MAX;

    fn create_bit_mask(bit_index: usize) -> Self {
        1 << bit_index
    }

    fn create_range_mask(start_bit: usize, bit_count: usize) -> Self {
        if bit_count == Self::BIT_COUNT {
            Self::MAX

        } else {
            ((1 << bit_count) - 1) << start_bit
        }
    }

    fn trailing_zeros(self) -> usize {
        Self::trailing_zeros(self) as usize
    }
    
}

impl BitStore for usize {

    const BIT_COUNT: usize = usize::BITS as usize;
    const ZERO: Self = 0;
    const MAX: Self = usize::MAX;

    fn create_bit_mask(bit_index: usize) -> Self {
        1 << bit_index
    }

    fn create_range_mask(start_bit: usize, bit_count: usize) -> Self {
        if bit_count == Self::BIT_COUNT {
            Self::MAX

        } else {
            ((1 << bit_count) - 1) << start_bit
        }
    }

    fn trailing_zeros(self) -> usize {
        Self::trailing_zeros(self) as usize
    }

}
