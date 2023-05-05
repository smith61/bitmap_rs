
use crate::slice::{BitmapSlice, BitmapSliceIter, BitmapSliceMut, BitmapSliceRangeIter};
use crate::store::BitStore;
use crate::traits::{BitmapOpts, BitmapOptsMut};

use std::marker::PhantomData;
use std::ops::Range;

///
/// Implements a bitmap over any type that can be converted to a reference to a slice.
/// This type is abstract over both the backing storage for the bitmap and the size
/// of individual elements in the slice.
/// 
/// Unlike a [BitmapSlice](crate::slice::BitmapSlice), this type supports both owning the
/// storage for the underlying bitmap and having that underlying storage change size.
/// This allows for a Bitmap instance to grow or shrink if the underlying storage
/// supports a dynamic size.
/// 
pub struct Bitmap<S: ?Sized, B = usize> {
    pub(super) _bs: PhantomData<*const B>,
    pub(super) bitmap_store: S
}

impl<S, B> Bitmap<S, B> {

    ///
    /// Creates a new bitmap with the provided backing store.
    /// 
    pub fn new(bitmap_store: S) -> Self {
        Bitmap { _bs: PhantomData::default(), bitmap_store }
    }

    ///
    /// Consumes this bitmap instance and returns the underlying storage.
    /// 
    pub fn into_inner(self) -> S {
        self.bitmap_store
    }

}

impl<S: AsRef<[B]> + ?Sized, B: BitStore> Bitmap<S, B> {

    ///
    /// A const containing the maximum supported length of the backing bitmap storage.
    /// 
    pub const MAXIMUM_BUFFER_SIZE: usize = usize::MAX / B::BIT_COUNT;
    
    ///
    /// Converts this bitmap into a [BitmapSlice](crate::slice::BitmapSlice) over the backing
    /// storage. The returned slice has the same length as this bitmap instance.
    /// 
    /// # Panics
    /// 
    /// Panics if the backing storage is larger than [MAXIMUM_BUFFER_SIZE](crate::bitmap::Bitmap::MAXIMUM_BUFFER_SIZE)
    /// 
    pub fn as_slice(&self) -> BitmapSlice<B> {
        let buffer = self.bitmap_store.as_ref();
        if buffer.len() > Self::MAXIMUM_BUFFER_SIZE {
            panic!("Bitmap buffer is too large ({} > {})", buffer.len(), Self::MAXIMUM_BUFFER_SIZE);
        }

        unsafe {
            BitmapSlice::new_unchecked(buffer, 0, buffer.len() * B::BIT_COUNT)
        }
    }

    ///
    /// Returns an iterator over all set bits in this bitmap.
    /// 
    pub fn iter(&self) -> BitmapSliceIter<B> {
        BitmapSliceIter::new(self.as_slice())
    }

    ///
    /// Returns an iterator over all ranges of set bits in this bitmap.
    /// 
    pub fn range_iter(&self) -> BitmapSliceRangeIter<B> {
        BitmapSliceRangeIter::new(self.as_slice())
    }

    ///
    /// This routine returns a [slice::BitmapSlice](BitmapSlice) starting at the first bit
    /// in the range (inclusive), and ending at the last bit in the range (exclusive).
    /// 
    pub fn subslice(&self, bit_range: Range<usize>) -> BitmapSlice<B> {
        BitmapSlice::new(self.bitmap_store.as_ref(), bit_range)
    }

    ///
    /// Returns a non-mutable reference to the underlying store.
    /// 
    pub fn store(&self) -> &S {
        &self.bitmap_store
    }

    ///
    /// Returns a mutable reference to the underlying store.
    /// 
    pub fn store_mut(&mut self) -> &mut S {
        &mut self.bitmap_store
    }

}

impl<S: AsRef<[B]> + ?Sized, B: BitStore> BitmapOpts for Bitmap<S, B> {
    
    fn find_next_clear_in_range(&self, range: Range<usize>) -> Option<usize> {
        self.as_slice().find_next_clear_in_range(range)
    }

    fn find_next_set_in_range(&self, range: Range<usize>) -> Option<usize> {
        self.as_slice().find_next_set_in_range(range)
    }

    fn get_bit(&self, bit_index: usize) -> bool {
        self.as_slice().get_bit(bit_index)
    }

    fn size(&self) -> usize {
        self.as_slice().size()
    }

}

impl<S: AsRef<[B]> + AsMut<[B]> + ?Sized, B: BitStore> Bitmap<S, B> {
    
    ///
    /// Converts this bitmap into a [BitmapSliceMut](crate::slice::BitmapSliceMut) over the backing
    /// storage. The returned slice has the same length as this bitmap instance.
    /// 
    /// # Panics
    /// 
    /// Panics if the backing storage is larger than [MAXIMUM_BUFFER_SIZE](crate::bitmap::Bitmap::MAXIMUM_BUFFER_SIZE)
    /// 
    pub fn as_slice_mut(&mut self) -> BitmapSliceMut<B> {
        let buffer = self.bitmap_store.as_mut();
        if buffer.len() > Self::MAXIMUM_BUFFER_SIZE {
            panic!("Bitmap buffer is too large ({} > {})", buffer.len(), Self::MAXIMUM_BUFFER_SIZE);
        }

        unsafe {
            BitmapSliceMut::new_unchecked(buffer, 0, buffer.len() * B::BIT_COUNT)
        }
    }

    ///
    /// This routine returns a [slice::BitmapSliceMut](BitmapSliceMut) starting at the
    /// first bit in the range (inclusive), and ending at the last bit in the range
    /// (exclusive).
    /// 
    pub fn subslice_mut(&mut self, bit_range: Range<usize>) -> BitmapSliceMut<B> {
        BitmapSliceMut::new(self.bitmap_store.as_mut(), bit_range)
    }

}

impl<S: AsRef<[B]> + AsMut<[B]> + ?Sized, B: BitStore> BitmapOptsMut for Bitmap<S, B> {

    ///
    /// This routine clears the bit at the provided index.
    /// 
    fn clear_bit(&mut self, bit_index: usize) {
        self.as_slice_mut().clear_bit(bit_index)
    }

    ///
    /// This routine clears the range of bits in the provided `bit_range`.
    /// 
    fn clear_bit_range(&mut self, bit_range: Range<usize>) {
        self.as_slice_mut().clear_bit_range(bit_range)
    }

    ///
    /// This routine sets the bit at the provided index.
    /// 
    fn set_bit(&mut self, bit_index: usize) {
        self.as_slice_mut().set_bit(bit_index)
    }

    ///
    /// This routine sets the range of bits in the provided `bit_range`.
    /// 
    fn set_bit_range(&mut self, bit_range: Range<usize>) {
        self.as_slice_mut().set_bit_range(bit_range)
    }

    ///
    /// This routine toggles the bit at the provided index.
    /// 
    fn toggle_bit(&mut self, bit_index: usize) {
        self.as_slice_mut().toggle_bit(bit_index)
    }

    ///
    /// This routine toggles the range of bits in the provided `bit_range`.
    /// 
    fn toggle_bit_range(&mut self, bit_range: Range<usize>) {
        self.as_slice_mut().toggle_bit_range(bit_range)
    }

}
