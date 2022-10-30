
use super::*;

#[test]
fn test_clear_bit_range() {
    let mut buffer = [0b11111111u8, 0b00001111, 0b11111111];

    BitmapSliceMut::new(&mut buffer, 3..14).clear_bit_range(1..11);
    assert_eq!(buffer, [0b00001111, 0b00000000, 0b11111111]);

    BitmapSliceMut::new(&mut buffer, 10..24).clear_bit_range(8..12);
    assert_eq!(buffer, [0b00001111, 0b00000000, 0b11000011]);

    BitmapSliceMut::new(&mut buffer, 0..24).clear_bit_range(0..24);
    assert_eq!(buffer, [0b00000000, 0b00000000, 0b00000000]);
}

#[test]
fn test_find_next_clear_range() {
    let buffer = [0b11110000u8, 0b11111111, 0b00001111];

    assert_eq!(BitmapSlice::new(&buffer, 0..buffer.len() * 8).find_first_clear_range(), Some((0, 4)));
    assert_eq!(BitmapSlice::new(&buffer, 2..10).find_first_clear_range(), Some((0, 2)));
    assert_eq!(BitmapSlice::new(&buffer, 1..10).find_first_clear_range_capped(2), Some((0, 2)));
    assert_eq!(BitmapSlice::new(&buffer, 4..10).find_first_clear_range(), None);

    assert_eq!(BitmapSlice::new(&buffer, 2..10).find_next_clear_range_from(4), None);
    assert_eq!(BitmapSlice::new(&buffer, 10..buffer.len() * 8 - 1).find_next_clear_range_from(11), Some((11, 2)));
    assert_eq!(BitmapSlice::new(&buffer, 10..buffer.len() * 8 - 1).find_next_clear_range_from_capped(11, 1), Some((11, 1)));
}

#[test]
fn test_find_next_set_range() {
    let buffer = [0b00001111u8, 0b00000000, 0b11110000];

    assert_eq!(BitmapSlice::new(&buffer, 0..24).find_first_set_range(), Some((0, 4)));
    assert_eq!(BitmapSlice::new(&buffer, 2..10).find_first_set_range(), Some((0, 2)));
    assert_eq!(BitmapSlice::new(&buffer, 1..10).find_first_set_range_capped(2), Some((0, 2)));
    assert_eq!(BitmapSlice::new(&buffer, 4..10).find_first_set_range(), None);

    assert_eq!(BitmapSlice::new(&buffer, 2..10).find_next_set_range_from(4), None);
    assert_eq!(BitmapSlice::new(&buffer, 10..23).find_next_set_range_from(11), Some((11, 2)));
    assert_eq!(BitmapSlice::new(&buffer, 10..23).find_next_set_range_from_capped(11, 1), Some((11, 1)));
}

#[test]
fn test_set_bit_range() {
    let mut buffer = [0b00000000u8, 0b11110000, 0b00000000];

    BitmapSliceMut::new(&mut buffer, 3..13).set_bit_range(1..9);
    assert_eq!(buffer, [0b11110000, 0b11111111, 0b00000000]);

    BitmapSliceMut::new(&mut buffer, 17..23).set_bit_range(1..5);
    assert_eq!(buffer, [0b11110000, 0b11111111, 0b00111100]);

    BitmapSliceMut::new(&mut buffer, 0..24).set_bit_range(0..24);
    assert_eq!(buffer, [0b11111111, 0b11111111, 0b11111111]);
}

#[test]
fn test_toggle_bit() {
    let mut buffer = [0b10101010u8, 0b11111111, 0b00000000];

    BitmapSliceMut::new(&mut buffer, 3..13).toggle_bit_range(1..9);
    assert_eq!(buffer, [0b01011010, 0b11110000, 0b00000000]);

    BitmapSliceMut::new(&mut buffer, 17..23).toggle_bit_range(1..5);
    assert_eq!(buffer, [0b01011010, 0b11110000, 0b00111100]);

    BitmapSliceMut::new(&mut buffer, 0..24).toggle_bit_range(0..24);
    assert_eq!(buffer, [0b10100101, 0b00001111, 0b11000011]);
}
