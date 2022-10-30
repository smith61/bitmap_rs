
use super::*;

#[test]
fn test_and_assign() {
    let mut buffer_1 = [0usize; 1];
    let mut buffer_2 = [0usize; 1];

    {
        let mut bitmap_1 = Bitmap::new(&mut buffer_1);
        let mut bitmap_2 = Bitmap::new(&mut buffer_2);
        
        bitmap_1.set_bit_range(0..32);
        bitmap_2.set_bit_range(16..48);
        bitmap_1 &= bitmap_2;
    }

    assert_eq!([0x00000000FFFF0000], buffer_1);
}

#[test]
fn test_or_assign() {
    let mut buffer_1 = [0usize; 1];
    let mut buffer_2 = [0usize; 1];

    {
        let mut bitmap_1 = Bitmap::new(&mut buffer_1);
        let mut bitmap_2 = Bitmap::new(&mut buffer_2);
        
        bitmap_1.set_bit_range(0..32);
        bitmap_2.set_bit_range(16..48);
        bitmap_1 |= bitmap_2;
    }

    assert_eq!([0x0000FFFFFFFFFFFF], buffer_1);
}

#[test]
fn test_xor_assign() {
    let mut buffer_1 = [0usize; 1];
    let mut buffer_2 = [0usize; 1];

    {
        let mut bitmap_1 = Bitmap::new(&mut buffer_1);
        let mut bitmap_2 = Bitmap::new(&mut buffer_2);
        
        bitmap_1.set_bit_range(0..32);
        bitmap_2.set_bit_range(16..48);
        bitmap_1 ^= bitmap_2;
    }

    assert_eq!([0x0000FFFF0000FFFF], buffer_1);
}

#[test]
fn test_clear_bit_range() {
    let mut buffer = [0b11111111u8, 0b00001111, 0b11111111];
    let mut bitmap = Bitmap::new(&mut buffer);

    bitmap.clear_bit_range(4..12);
    assert_eq!(*bitmap.store(), &[0b00001111, 0b00000000, 0b11111111]);

    bitmap.clear_bit_range(18..22);
    assert_eq!(*bitmap.store(), &[0b00001111, 0b00000000, 0b11000011]);

    bitmap.clear_bit_range(0..bitmap.size());
    assert_eq!(*bitmap.store(), &[0b00000000, 0b00000000, 0b00000000]);
}

#[test]
fn test_find_next_clear_range() {
    let buffer = [0b11110000u8, 0b11111111, 0b00001111];
    let bitmap = Bitmap::new(&buffer);

    assert_eq!(bitmap.find_first_clear_range(), Some((0, 4)));
    assert_eq!(bitmap.find_first_clear_range_capped(2), Some((0, 2)));

    assert_eq!(bitmap.find_next_clear_range_from(0), Some((0, 4)));
    assert_eq!(bitmap.find_next_clear_range_from_capped(0, 2), Some((0, 2)));
    assert_eq!(bitmap.find_next_clear_range_from(2), Some((2, 2)));
    assert_eq!(bitmap.find_next_clear_range_from_capped(2, 10), Some((2, 2)));
    assert_eq!(bitmap.find_next_clear_range_from_capped(2, 1), Some((2, 1)));
    assert_eq!(bitmap.find_next_clear_range_from(5), Some((20, 4)));
    assert_eq!(bitmap.find_next_clear_range_from_capped(5, 2), Some((20, 2)));
    assert_eq!(bitmap.find_next_clear_range_from(24), None);
}

#[test]
fn test_find_next_set_range() {
    let buffer = [0b00001111u8, 0b00000000, 0b11110000];
    let bitmap = Bitmap::new(&buffer);

    assert_eq!(bitmap.find_first_set_range(), Some((0, 4)));
    assert_eq!(bitmap.find_first_set_range_capped(2), Some((0, 2)));

    assert_eq!(bitmap.find_next_set_range_from(0), Some((0, 4)));
    assert_eq!(bitmap.find_next_set_range_from_capped(0, 2), Some((0, 2)));
    assert_eq!(bitmap.find_next_set_range_from(2), Some((2, 2)));
    assert_eq!(bitmap.find_next_set_range_from_capped(2, 10), Some((2, 2)));
    assert_eq!(bitmap.find_next_set_range_from_capped(2, 1), Some((2, 1)));
    assert_eq!(bitmap.find_next_set_range_from(5), Some((20, 4)));
    assert_eq!(bitmap.find_next_set_range_from_capped(5, 2), Some((20, 2)));
    assert_eq!(bitmap.find_next_set_range_from(24), None);
}

#[test]
fn test_get_bit() {
    let buffer = [0b10101010u8, 0b11111111, 0b10000000];
    let bitmap = Bitmap::new(&buffer);

    assert_eq!(bitmap.get_bit(0), false);
    assert_eq!(bitmap.get_bit(1), true);
    assert_eq!(bitmap.get_bit(2), false);
    assert_eq!(bitmap.get_bit(3), true);

    assert_eq!(bitmap.get_bit(10), true);
    assert_eq!(bitmap.get_bit(11), true);
    assert_eq!(bitmap.get_bit(12), true);

    assert_eq!(bitmap.get_bit(16), false);
    assert_eq!(bitmap.get_bit(22), false);
    assert_eq!(bitmap.get_bit(23), true);
}

#[test]
fn test_set_bit() {
    let mut buffer = [0u8; 3];
    let mut bitmap = Bitmap::new(&mut buffer);

    bitmap.set_bit(0);
    bitmap.set_bit(2);
    bitmap.set_bit(4);
    bitmap.set_bit(6);
    for index in 8..16 {
        bitmap.set_bit(index);
    }
    
    bitmap.set_bit(bitmap.size() - 1);

    assert_eq!(buffer, [0b01010101, 0b11111111, 0b10000000]);
}

#[test]
fn test_set_bit_range() {
    let mut buffer = [0b00000000u8, 0b11110000, 0b00000000];
    let mut bitmap = Bitmap::new(&mut buffer);

    bitmap.set_bit_range(4..12);
    assert_eq!(*bitmap.store(), &[0b11110000, 0b11111111, 0b00000000]);

    bitmap.set_bit_range(18..22);
    assert_eq!(*bitmap.store(), &[0b11110000, 0b11111111, 0b00111100]);

    bitmap.set_bit_range(0..bitmap.size());
    assert_eq!(*bitmap.store(), &[0b11111111, 0b11111111, 0b11111111]);
}

#[test]
fn test_toggle_bit() {
    let mut buffer = [0b10101010u8, 0b11111111, 0b00000000];
    let mut bitmap = Bitmap::new(&mut buffer);

    bitmap.toggle_bit_range(4..12);
    assert_eq!(*bitmap.store(), &[0b01011010, 0b11110000, 0b00000000]);

    bitmap.toggle_bit_range(18..22);
    assert_eq!(*bitmap.store(), &[0b01011010, 0b11110000, 0b00111100]);

    bitmap.toggle_bit_range(0..bitmap.size());
    assert_eq!(*bitmap.store(), &[0b10100101, 0b00001111, 0b11000011]);
}
