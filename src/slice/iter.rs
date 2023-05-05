
use super::BitmapSliceImpl;

use crate::polyfill::Const;
use crate::store::BitStore;
use crate::traits::BitmapOpts;

///
/// An iterator over each set bit in a bitmap slice.
/// 
pub struct BitmapSliceIter<'a, B: BitStore> {
    inner: BitmapSliceRangeIter<'a, B>,
    last_range: Option<(usize, usize)>
}

impl<'a, B: BitStore> BitmapSliceIter<'a, B> {
    
    pub(crate) fn new(inner: BitmapSliceImpl<'a, B, Const>) -> Self {
        BitmapSliceIter { inner: BitmapSliceRangeIter::new(inner), last_range: None }
    }

}

impl<'a, B: BitStore> Iterator for BitmapSliceIter<'a, B> {

    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_range.is_none() {
            self.last_range = self.inner.next();
        }

        if let Some((range_start, range_count)) = self.last_range.as_mut() {
            debug_assert!(*range_count != 0);

            let result = *range_start;
            *range_start += 1;
            *range_count -= 1;
            if *range_count == 0 {
                self.last_range.take();
            }

            Some(result)

        } else {
            None
        }
    }

}

///
/// An iterator over each range of set bits in a bitmap slice.
/// 
pub struct BitmapSliceRangeIter<'a, B: BitStore> {
    inner: BitmapSliceImpl<'a, B, Const>,
    last_range_end: usize
}

impl<'a, B: BitStore> BitmapSliceRangeIter<'a, B> {
    
    pub(crate) fn new(inner: BitmapSliceImpl<'a, B, Const>) -> Self {
        BitmapSliceRangeIter { inner, last_range_end: 0 }
    }

}

impl<'a, B: BitStore> Iterator for BitmapSliceRangeIter<'a, B> {

    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_range_end < self.inner.size() {
            if let Some(next_range) = self.inner.find_next_set_range_from(self.last_range_end) {
                self.last_range_end = next_range.0 + next_range.1;
                Some(next_range)

            } else {
                self.last_range_end = self.inner.size();
                None
            }

        } else {
            None
        }
    }

}
