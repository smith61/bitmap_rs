
use super::BitmapSliceImpl;
use crate::polyfill::{Mut, Mutability};
use crate::store::BitStore;
use crate::traits::BitmapOpts;

use std::marker::PhantomData;
use std::ops::Range;
use std::ptr::{self, NonNull};

#[derive(Clone, Copy, Debug)]
pub(super) enum BitmapSliceOperation {
    Clear,
    Set,
    Toggle
}

impl BitmapSliceOperation {

    #[inline(always)]
    pub(super) unsafe fn apply<B: BitStore>(&self, target: *mut B, mask: B) {
        match self {
            BitmapSliceOperation::Clear => *target &= !mask,
            BitmapSliceOperation::Set => *target |= mask,
            BitmapSliceOperation::Toggle => *target ^= mask
        }
    }

}

impl<'a, B: BitStore, M: Mutability> BitmapSliceImpl<'a, B, M> {

    pub(super) fn find_next_in_range<const CLEAR_BIT: bool>(&self, range: Range<usize>) -> Option<usize> {
        if range.is_empty() {
            return None;

        } else {
            let total_bit_count = self.size();
            if (range.end - range.start) > total_bit_count {
                panic!("Invalid bit range [{}:{}] for bitmap of size {}",
                       range.start,
                       range.end,
                       total_bit_count);
            }
        }

        let (starting_slot, starting_offset) = self.translate_bit_index(range.start);
        let ending_bit = range.end + (self.first_bit_offset as usize);
        let ending_slot = crate::polyfill::div_ceil(ending_bit, B::BIT_COUNT);

        let mut current_slot = starting_slot;
        let mut buffer = unsafe { self.buffer_address.as_ptr().add(starting_slot) };
        while current_slot < ending_slot {
            let current_bits = {
                let mut current_bits = unsafe { ptr::read(buffer) };

                if current_slot == starting_slot {
                    let mask = B::create_range_mask(0, starting_offset);
                    if CLEAR_BIT {
                        current_bits |= mask;

                    } else {
                        current_bits &= !mask;
                    }
                }

                if CLEAR_BIT {
                    current_bits = !current_bits;
                }

                current_bits
            };

            if current_bits != B::ZERO {
                let mut first_matching_bit = current_slot * B::BIT_COUNT;
                first_matching_bit += current_bits.trailing_zeros() as usize;
                first_matching_bit -= self.first_bit_offset as usize;
                if first_matching_bit < ending_bit {
                    return Some(first_matching_bit);

                } else {
                    return None;
                }
            }

            current_slot += 1;
            buffer = unsafe { buffer.add(1) };
        }

        None
    }

    pub(super) unsafe fn from_raw_parts(buffer_address: NonNull<B>, first_bit_offset: u8, bit_count: usize) -> Self {
        
        debug_assert!((first_bit_offset as usize) < B::BIT_COUNT);

        BitmapSliceImpl {
            buffer_address,
            bit_count,
            first_bit_offset,
            _lt: PhantomData::default(),
            _mut: PhantomData::default()
        }
    }

    pub(super) fn translate_bit_index(&self, bit_index: usize) -> (usize, usize) {
        if bit_index >= self.size() {
            panic!("Overlow when accessing bit index {}", bit_index);
        }

        let real_bit_index = bit_index + (self.first_bit_offset as usize);
        (real_bit_index / B::BIT_COUNT, real_bit_index % B::BIT_COUNT)
    }

}

impl<'a, B: BitStore> BitmapSliceImpl<'a, B, Mut> {
    
    #[inline(always)]
    pub(super) fn modify_bit(&mut self, bit_index: usize, operation: BitmapSliceOperation) {
        let (slot, offset) = self.translate_bit_index(bit_index);
        unsafe {
            operation.apply(self.buffer_address.as_ptr().add(slot), B::create_bit_mask(offset));
        }
    }

    #[inline(always)]
    pub(super) fn modify_bit_range(&mut self, bit_range: Range<usize>, operation: BitmapSliceOperation) {
        if bit_range.is_empty() {
            return;
        }

        if (bit_range.start >= self.size()) ||
           (bit_range.end > self.size()) {

            panic!("Invalid bit range [{}:{}] for bitmap of size {}",
                   bit_range.start,
                   bit_range.end,
                   self.size());
        }

        let (starting_slot, starting_offset) = self.translate_bit_index(bit_range.start);

        let mut buffer = unsafe { self.buffer_address.as_ptr().add(starting_slot) };

        let mut current_offset = starting_offset;
        let mut current_count = B::BIT_COUNT - current_offset;
        let mut current_mask = B::create_range_mask(current_offset, current_count);
        let mut remaining = bit_range.count();
        while remaining >= current_count {
            unsafe { operation.apply(buffer, current_mask) };

            remaining -= current_count;
            current_offset = 0;
            current_count = B::BIT_COUNT;
            current_mask = B::MAX;
            buffer = unsafe { buffer.add(1) };
        }

        if remaining != 0 {
            unsafe { operation.apply(buffer, B::create_range_mask(current_offset, remaining)); }
        }
    }

}