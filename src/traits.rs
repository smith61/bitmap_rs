
use std::ops::Range;

pub trait BitmapOpts {

    ///
    /// This routine returns the zero based index of the first clear bit in the bitmap.
    /// If this slice does not contain any clear bits, None is returned.
    /// 
    fn find_first_clear(&self) -> Option<usize> {
        self.find_next_clear_from(0)
    }

    ///
    /// This routine returns a tuple containing the zero based index of the first clear bit and the
    /// total count of contigious clear bits starting at that index. If this slice does not contain
    /// any clear bits, None is returned.
    ///
    fn find_first_clear_range(&self) -> Option<(usize, usize)> {
        self.find_next_clear_range_from(0)
    }

    ///
    /// This routine returns a tuple containing the zero based index of the first clear bit and the
    /// total count of contigious clear bits starting at that index capped to `maximum_run_length`.
    /// If this slice does not contain any clear bits, None is returned.
    ///
    fn find_first_clear_range_capped(&self, maximum_run_length: usize) -> Option<(usize, usize)> {
        self.find_next_clear_range_from_capped(0, maximum_run_length)
    }

    ///
    /// This routine returns the zero based index of the first clear bit in the slice starting at
    /// the provided `starting_bit`. If this slice does not contain any clear bits starting at
    /// `starting_bit`, None is returned.
    /// 
    fn find_next_clear_from(&self, starting_bit: usize) -> Option<usize> {
        self.find_next_clear_in_range(starting_bit..self.size())
    }

    fn find_next_clear_in_range(&self, range: Range<usize>) -> Option<usize>;

    ///
    /// This routine returns a tuple containing the zero based index of the first clear bit starting at
    /// the provided `starting_bit` and the total count of contigious clear bits starting at that index.
    /// If this slice does not contain any clear bits starting at `starting_bit`, None is returned.
    ///
    fn find_next_clear_range_from(&self, starting_bit: usize) -> Option<(usize, usize)> {
        self.find_next_clear_range_from_capped(starting_bit, usize::MAX)
    }

    ///
    /// This routine returns a tuple containing the zero based index of the first clear bit starting at
    /// the provided `starting_bit` and the total count of contigious clear bits starting at that index
    /// capped to `maximum_run_length`. If this slice does not contain any clear bits starting at
    /// `starting_bit`, None is returned.
    ///
    fn find_next_clear_range_from_capped(&self, starting_bit: usize, maximum_run_length: usize) -> Option<(usize, usize)> {
        self.find_next_clear_in_range(starting_bit..self.size())
            .map(|first_clear_bit| {
                let maximum_run_length = std::cmp::min(maximum_run_length, self.size() - first_clear_bit);
                let next_set_bit =
                    self.find_next_set_in_range((first_clear_bit + 1)..(first_clear_bit + maximum_run_length))
                        .unwrap_or(first_clear_bit + maximum_run_length);

                (first_clear_bit, next_set_bit - first_clear_bit)
            })
    }

    ///
    /// This routine returns the zero based index of the first set bit in the slice.
    /// If this slice does not contain any set bits, None is returned.
    /// 
    fn find_first_set(&self) -> Option<usize> {
        self.find_next_set_from(0)
    }

    ///
    /// This routine returns a tuple containing the zero based index of the first set bit and the
    /// total count of contigious set bits starting at that index. If this slice does not contain
    /// any set bits, None is returned.
    ///
    fn find_first_set_range(&self) -> Option<(usize, usize)> {
        self.find_next_set_range_from(0)
    }

    ///
    /// This routine returns a tuple containing the zero based index of the first set bit and the
    /// total count of contigious set bits starting at that index capped to `maximum_run_length`.
    /// If this slice does not contain any set bits, None is returned.
    ///
    fn find_first_set_range_capped(&self, maximum_run_length: usize) -> Option<(usize, usize)> {
        self.find_next_set_range_from_capped(0, maximum_run_length)
    }

    ///
    /// This routine returns the zero based index of the first set bit in the slice starting at
    /// the provided `starting_bit`. If this slice does not contain any set bits starting at
    /// `starting_bit`, None is returned.
    /// 
    fn find_next_set_from(&self, starting_bit: usize) -> Option<usize> {
        self.find_next_set_in_range(starting_bit..self.size())
    }

    fn find_next_set_in_range(&self, range: Range<usize>) -> Option<usize>;

    ///
    /// This routine returns a tuple containing the zero based index of the first set bit starting at
    /// the provided `starting_bit` and the total count of contigious set bits starting at that index.
    /// If this slice does not contain any set bits starting at `starting_bit`, None is returned.
    ///
    fn find_next_set_range_from(&self, starting_bit: usize) -> Option<(usize, usize)> {
        self.find_next_set_range_from_capped(starting_bit, usize::MAX)
    }

    ///
    /// This routine returns a tuple containing the zero based index of the first set bit starting at
    /// the provided `starting_bit` and the total count of contigious set bits starting at that index
    /// capped to `maximum_run_length`. If this slice does not contain any set bits starting at
    /// `starting_bit`, None is returned.
    ///
    fn find_next_set_range_from_capped(&self, starting_bit: usize, maximum_run_length: usize) -> Option<(usize, usize)> {
        self.find_next_set_in_range(starting_bit..self.size())
            .map(|first_set_bit| {
                let maximum_run_length = std::cmp::min(maximum_run_length, self.size() - first_set_bit);
                let next_clear_bit =
                    self.find_next_clear_in_range((first_set_bit + 1)..(first_set_bit + maximum_run_length))
                        .unwrap_or(first_set_bit + maximum_run_length);

                (first_set_bit, next_clear_bit - first_set_bit)
            })
    }

    ///
    /// This routine returns `true` if the bit at the provided index is set, otherwise returns false.
    /// 
    fn get_bit(&self, bit_index: usize) -> bool;

    ///
    /// This routine returns the total size in bits of this slice.
    /// 
    fn size(&self) -> usize;

}

pub trait BitmapOptsMut : BitmapOpts {

    ///
    /// This routine clears the bit at the provided index.
    /// 
    fn clear_bit(&mut self, bit_index: usize);

    ///
    /// This routine clears the range of bits in the provided `bit_range`.
    /// 
    fn clear_bit_range(&mut self, bit_range: Range<usize>);

    ///
    /// This routine sets the bit at the provided index.
    /// 
    fn set_bit(&mut self, bit_index: usize);

    ///
    /// This routine sets the range of bits in the provided `bit_range`.
    /// 
    fn set_bit_range(&mut self, bit_range: Range<usize>);
    
    ///
    /// This routine toggles the bit at the provided index.
    /// 
    fn toggle_bit(&mut self, bit_index: usize);

    ///
    /// This routine toggles the range of bits in the provided `bit_range`.
    /// 
    fn toggle_bit_range(&mut self, bit_range: Range<usize>);

}
