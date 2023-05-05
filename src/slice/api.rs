
use crate::store::BitStore;
use crate::traits::{BitmapOpts, BitmapOptsMut};
use crate::polyfill::{Const, Mut, Mutability};

use super::{BitmapSliceIter, BitmapSliceRangeIter};
use super::internal::BitmapSliceOperation;

use std::marker::PhantomData;
use std::ops::Range;
use std::ptr::NonNull;

///
/// Implements a bitmap slice over a subslice of a bitmap. A bitmap slice can be
/// mutable, if the provided storage is mutable and can be split or shrunk as
/// needed. A bitmap slice does not support owning the underlying storage.
/// 
pub struct BitmapSliceImpl<'a, B: BitStore, M: Mutability> {
    pub(super) buffer_address: NonNull<B>,
    pub(super) bit_count: usize,
    pub(super) first_bit_offset: u8,
    pub(super) _lt: PhantomData<(&'a [B], &'a mut [B])>,
    pub(super) _mut: PhantomData<M>
}

impl<'a, B: BitStore> Copy for BitmapSliceImpl<'a, B, Const> { }
impl<'a, B: BitStore> Clone for BitmapSliceImpl<'a, B, Const> {

    fn clone(&self) -> Self {
        unsafe {
            Self::from_raw_parts(self.buffer_address, self.first_bit_offset, self.bit_count)
        }
    }

}

impl<'a, B: BitStore> BitmapSliceImpl<'a, B, Const> {

    ///
    /// Creates a new non-mutable slice over the provided storage covering the
    /// provided range.
    /// 
    pub fn new(mut buffer: &'a [B], bit_range: Range<usize>) -> Self {
        if bit_range.start > bit_range.end {
            panic!("Invalid bit range start ({}) > end ({})", bit_range.start, bit_range.end);

        } else {
            let starting_slot = bit_range.start / B::BIT_COUNT;
            let ending_slot = crate::polyfill::div_ceil(bit_range.end, B::BIT_COUNT);
            if (starting_slot >= bit_range.len()) ||
               (ending_slot > bit_range.len()) {

                panic!("Invalid bit range [{}:{}] for buffer of size {}",
                       starting_slot,
                       ending_slot,
                       buffer.len());
            }

            buffer = &buffer[starting_slot..ending_slot];
        }

        let first_bit_offset = (bit_range.start % B::BIT_COUNT) as u8;
        unsafe {
            let buffer_address = NonNull::new_unchecked(buffer.as_ptr() as *mut _);
            Self::from_raw_parts(buffer_address, first_bit_offset, bit_range.count())
        }
    }

    ///
    /// Creates a new non-mutable slice over the provided storage. `first_bit_offset` must be
    /// less than `B::BIT_COUNT` and `bit_count` must be less than or equal to
    /// `buffer.len() * B::BIT_COUNT - first_bit_offset`. These conditions are not checked
    /// and hence this routine is marked as unsafe.
    /// 
    pub unsafe fn new_unchecked(buffer: &'a [B], first_bit_offset: u8, bit_count: usize) -> Self {
        let buffer_address = NonNull::new_unchecked(buffer.as_ptr() as *mut _);

        debug_assert!((first_bit_offset as usize) < B::BIT_COUNT);

        Self::from_raw_parts(buffer_address, first_bit_offset, bit_count)
    }

}

impl<'a, B: BitStore> BitmapSliceImpl<'a, B, Mut> {

    ///
    /// Creates a new mutable slice over the provided storage covering the
    /// provided range.
    /// 
    pub fn new(mut buffer: &'a mut [B], bit_range: Range<usize>) -> Self {
        if bit_range.start > bit_range.end {
            panic!("Invalid bit range start ({}) > end ({})", bit_range.start, bit_range.end);

        } else {
            let starting_slot = bit_range.start / B::BIT_COUNT;
            let ending_slot = crate::polyfill::div_ceil(bit_range.end, B::BIT_COUNT);
            if (starting_slot >= bit_range.len()) ||
               (ending_slot > bit_range.len()) {

                panic!("Invalid bit range [{}:{}] for buffer of size {}",
                       starting_slot,
                       ending_slot,
                       buffer.len());
            }

            buffer = &mut buffer[starting_slot..ending_slot];
        }

        let first_bit_offset = (bit_range.start % B::BIT_COUNT) as u8;
        unsafe {
            let buffer_address = NonNull::new_unchecked(buffer.as_mut_ptr());
            Self::from_raw_parts(buffer_address, first_bit_offset, bit_range.count())
        }
    }

    ///
    /// Creates a new mutable slice over the provided storage. `first_bit_offset` must be
    /// less than `B::BIT_COUNT` and `bit_count` must be less than or equal to
    /// `buffer.len() * B::BIT_COUNT - first_bit_offset`. These conditions are not checked
    /// and hence this routine is marked as unsafe.
    /// 
    pub unsafe fn new_unchecked(buffer: &'a mut [B], first_bit_offset: u8, bit_count: usize) -> Self {
        let buffer_address = NonNull::new_unchecked(buffer.as_mut_ptr());

        debug_assert!((first_bit_offset as usize) < B::BIT_COUNT);

        Self::from_raw_parts(buffer_address, first_bit_offset, bit_count)
    }

}

impl<'a, B: BitStore, M: Mutability> BitmapSliceImpl<'a, B, M> {

    ///
    /// Temporarily downgrades this potentially mutable slice into a non-mutable
    /// slice over the same range of bits.
    /// 
    pub fn as_const(&self) -> BitmapSliceImpl<B, Const> {
        unsafe {
            BitmapSliceImpl::from_raw_parts(self.buffer_address, self.first_bit_offset, self.bit_count)
        }
    }

    ///
    /// Returns an iterator over all set bits in this slice.
    /// 
    pub fn iter(&self) -> BitmapSliceIter<B> {
        BitmapSliceIter::new(self.as_const())
    }

    ///
    /// Returns an iterator over all ranges of set bits in this slice.
    /// 
    pub fn range_iter(&self) -> BitmapSliceRangeIter<B> {
        BitmapSliceRangeIter::new(self.as_const())
    }

    ///
    /// This routine splits this bitmap slice into two non-mutable subslices. The
    /// first slice starts at the same bit as this slice and ends at `bit_index` (exclusive).
    /// The second slice starts `bit_index` (inclusive) and ends at the same bit
    /// as this slice.
    /// 
    pub fn split_at(self, bit_index: usize) -> (BitmapSliceImpl<'a, B, Const>, BitmapSliceImpl<'a, B, Const>) {
        if bit_index > self.bit_count {
            panic!("Invalid bit index ({} > {})", bit_index, self.bit_count);
        }

        let first_slice = unsafe {
            BitmapSliceImpl::from_raw_parts(self.buffer_address, self.first_bit_offset, bit_index)
        };

        let second_slice = unsafe {
            let real_bit_index = bit_index + (self.first_bit_offset as usize);
            let real_starting_slot = real_bit_index / B::BIT_COUNT;
            let real_first_bit_offset = (real_bit_index % B::BIT_COUNT) as u8;

            let buffer_address = {
                let mut buffer_address = self.buffer_address.as_ptr();
                buffer_address = buffer_address.add(real_starting_slot);
                NonNull::new_unchecked(buffer_address)
            };

            let remaining_bit_count = self.bit_count - bit_index;

            BitmapSliceImpl::from_raw_parts(buffer_address, real_first_bit_offset, remaining_bit_count)
        };

        (first_slice, second_slice)
    }

    ///
    /// This routine returns a [BitmapSlice](crate::slice::BitmapSlice) starting at the first bit
    /// in the range (inclusive), and ending at the last bit in the range (exclusive).
    /// 
    pub fn subslice(&self, bit_range: Range<usize>) -> BitmapSliceImpl<B, Const> {
        let (bit_start, bit_end, bit_count) = (bit_range.start, bit_range.end, bit_range.count());
        if bit_start > bit_end {
            panic!("Invalid bit range start ({}) > end ({})", bit_start, bit_end);

        } else if bit_count > self.bit_count {
            panic!("Invalid bit range [{}:{}] for bit map slice of size {}",
                   bit_start,
                   bit_end,
                   self.bit_count);
        }

        let real_bit_start = bit_start + (self.first_bit_offset as usize);
        let real_starting_slot = real_bit_start / B::BIT_COUNT;
        let real_first_bit_offset = (real_bit_start % B::BIT_COUNT) as u8;

        unsafe {
            let buffer_address = {
                let mut buffer_address = self.buffer_address.as_ptr();
                buffer_address = buffer_address.add(real_starting_slot);
                NonNull::new_unchecked(buffer_address)
            };

            BitmapSliceImpl::<B, Const>::from_raw_parts(buffer_address, real_first_bit_offset, bit_count)
        }
    }

    ///
    /// Converts this slice into a const slice.
    /// 
    pub fn to_const_slice(self) -> BitmapSliceImpl<'a, B, Const> {
        unsafe {
            BitmapSliceImpl::<'a, B, Const>::from_raw_parts(self.buffer_address, self.first_bit_offset, self.bit_count)
        }
    }

}

impl<'a, B: BitStore, M: Mutability> BitmapOpts for BitmapSliceImpl<'a, B, M> {
    
    fn find_next_clear_in_range(&self, range: Range<usize>) -> Option<usize> {
        self.find_next_in_range::<true>(range)
    }

    fn find_next_set_in_range(&self, range: Range<usize>) -> Option<usize> {
        self.find_next_in_range::<false>(range)
    }

    fn get_bit(&self, bit_index: usize) -> bool {
        let (slot, offset) = self.translate_bit_index(bit_index);
        let slot_contents = 
            unsafe { self.buffer_address.as_ptr().add(slot).read() };

        (slot_contents & B::create_bit_mask(offset)) != B::ZERO
    }

    fn size(&self) -> usize {
        self.bit_count
    }
    
}

impl<'a, B: BitStore> BitmapSliceImpl<'a, B, Mut> {

    ///
    /// This routine splits this bitmap slice into two mutable subslices. The first
    /// slice starts at the same bit as this slice and ends at `bit_index` (exclusive).
    /// The second slice starts `bit_index` (inclusive) and ends at the same bit
    /// as this slice.
    /// 
    pub fn split_at_mut(self, bit_index: usize) -> (BitmapSliceImpl<'a, B, Mut>, BitmapSliceImpl<'a, B, Mut>) {
        if bit_index > self.bit_count {
            panic!("Invalid bit index ({} > {})", bit_index, self.bit_count);
        }

        let first_slice = unsafe {
            BitmapSliceImpl::from_raw_parts(self.buffer_address, self.first_bit_offset, bit_index)
        };

        let second_slice = unsafe {
            let real_bit_index = bit_index + (self.first_bit_offset as usize);
            let real_starting_slot = real_bit_index / B::BIT_COUNT;
            let real_first_bit_offset = (real_bit_index % B::BIT_COUNT) as u8;

            let buffer_address = {
                let mut buffer_address = self.buffer_address.as_ptr();
                buffer_address = buffer_address.add(real_starting_slot);
                NonNull::new_unchecked(buffer_address)
            };

            let remaining_bit_count = self.bit_count - bit_index;

            BitmapSliceImpl::from_raw_parts(buffer_address, real_first_bit_offset, remaining_bit_count)
        };

        (first_slice, second_slice)
    }

    ///
    /// This routine returns a [BitmapSliceMut](crate::slice::BitmapSliceMut) starting at the
    /// first bit in the range (inclusive), and ending at the last bit in the range
    /// (exclusive).
    /// 
    pub fn subslice_mut(&mut self, bit_range: Range<usize>) -> BitmapSliceImpl<B, Mut> {
        let (bit_start, bit_end, bit_count) = (bit_range.start, bit_range.end, bit_range.count());
        if bit_start > bit_end {
            panic!("Invalid bit range start ({}) > end ({})", bit_start, bit_end);

        } else if bit_count > self.bit_count {
            panic!("Invalid bit range [{}:{}] for bit map slice of size {}",
                   bit_start,
                   bit_end,
                   self.bit_count);
        }

        let real_bit_start = bit_start + (self.first_bit_offset as usize);
        let real_starting_slot = real_bit_start / B::BIT_COUNT;
        let real_first_bit_offset = (real_bit_start % B::BIT_COUNT) as u8;

        unsafe {
            let buffer_address = {
                let mut buffer_address = self.buffer_address.as_ptr();
                buffer_address = buffer_address.add(real_starting_slot);
                NonNull::new_unchecked(buffer_address)
            };

            BitmapSliceImpl::<B, Mut>::from_raw_parts(buffer_address, real_first_bit_offset, bit_count)
        }
    }

}

impl<'a, B: BitStore> BitmapOptsMut for BitmapSliceImpl<'a, B, Mut> {
    
    ///
    /// This routine clears the bit at the provided index.
    /// 
    fn clear_bit(&mut self, bit_index: usize) {
        self.modify_bit(bit_index, BitmapSliceOperation::Clear);
    }

    ///
    /// This routine clears the range of bits in the provided `bit_range`.
    /// 
    fn clear_bit_range(&mut self, bit_range: Range<usize>) {
        self.modify_bit_range(bit_range, BitmapSliceOperation::Clear);
    }

    ///
    /// This routine sets the bit at the provided index.
    /// 
    fn set_bit(&mut self, bit_index: usize) {
        self.modify_bit(bit_index, BitmapSliceOperation::Set);
    }

    ///
    /// This routine sets the range of bits in the provided `bit_range`.
    /// 
    fn set_bit_range(&mut self, bit_range: Range<usize>) {
        self.modify_bit_range(bit_range, BitmapSliceOperation::Set);
    }
    
    ///
    /// This routine toggles the bit at the provided index.
    /// 
    fn toggle_bit(&mut self, bit_index: usize) {
        self.modify_bit(bit_index, BitmapSliceOperation::Toggle);
    }

    ///
    /// This routine toggles the range of bits in the provided `bit_range`.
    /// 
    fn toggle_bit_range(&mut self, bit_range: Range<usize>) {
        self.modify_bit_range(bit_range, BitmapSliceOperation::Toggle);
    }

}
