
use super::Bitmap;

use crate::store::BitStore;

use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

impl<B: BitStore, S: AsRef<[B]> + AsMut<[B]>, O: AsRef<[B]>> BitAndAssign<Bitmap<O, B>> for Bitmap<S, B> {

    fn bitand_assign(&mut self, rhs: Bitmap<O, B>) {
        *self &= &rhs;
    }

}

impl<B: BitStore, S: AsRef<[B]> + AsMut<[B]>, O: AsRef<[B]>> BitAndAssign<&Bitmap<O, B>> for Bitmap<S, B> {

    fn bitand_assign(&mut self, rhs: &Bitmap<O, B>) {
        self.bitmap_store
            .as_mut()
            .iter_mut()
            .zip(rhs.bitmap_store.as_ref().iter())
            .for_each(|(dest, src)| *dest &= *src);
    }

}

impl<B: BitStore, S: AsRef<[B]> + AsMut<[B]>, O: AsRef<[B]>> BitOrAssign<Bitmap<O, B>> for Bitmap<S, B> {

    fn bitor_assign(&mut self, rhs: Bitmap<O, B>) {
        *self |= &rhs;
    }

}

impl<B: BitStore, S: AsRef<[B]> + AsMut<[B]>, O: AsRef<[B]>> BitOrAssign<&Bitmap<O, B>> for Bitmap<S, B> {

    fn bitor_assign(&mut self, rhs: &Bitmap<O, B>) {
        self.bitmap_store
            .as_mut()
            .iter_mut()
            .zip(rhs.bitmap_store.as_ref().iter())
            .for_each(|(dest, src)| *dest |= *src);
    }

}

impl<B: BitStore, S: AsRef<[B]> + AsMut<[B]>, O: AsRef<[B]>> BitXorAssign<Bitmap<O, B>> for Bitmap<S, B> {

    fn bitxor_assign(&mut self, rhs: Bitmap<O, B>) {
        *self ^= &rhs;
    }

}

impl<B: BitStore, S: AsRef<[B]> + AsMut<[B]>, O: AsRef<[B]>> BitXorAssign<&Bitmap<O, B>> for Bitmap<S, B> {

    fn bitxor_assign(&mut self, rhs: &Bitmap<O, B>) {
        self.bitmap_store
            .as_mut()
            .iter_mut()
            .zip(rhs.bitmap_store.as_ref().iter())
            .for_each(|(dest, src)| *dest ^= *src);
    }

}
