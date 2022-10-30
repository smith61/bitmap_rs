
mod api;
mod iter;
mod internal;

#[cfg(test)]
mod test;

use crate::polyfill::{Const, Mut};

pub use self::api::BitmapSliceImpl;
pub use self::iter::{BitmapSliceIter, BitmapSliceRangeIter};

///
/// Alias for a non-mutable [slice::BitmapSliceImpl](BitmapSliceImpl).
/// 
pub type BitmapSlice<'a, B = usize> = BitmapSliceImpl<'a, B, Const>;

///
/// Alias for a mutable [slice::BitmapSliceImpl](BitmapSliceImpl).
/// 
pub type BitmapSliceMut<'a, B = usize> = BitmapSliceImpl<'a, B, Mut>;
