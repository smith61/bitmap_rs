
mod seal {

    pub trait Sealed {}

}

///
/// This trait represents the mutability of a type and allows code to be generic
/// over the mutability of the contained type.
/// 
pub trait Mutability: self::seal::Sealed {

    ///
    /// A constant bool indicating if this type is [Mut]
    /// 
    /// # Examples
    /// ```
    /// # use bitmap::prelude::*;
    /// assert_eq!(true, <Mut as Mutability>::IS_MUTABLE);
    /// assert_eq!(false, <Const as Mutability>::IS_MUTABLE);
    /// ```
    const IS_MUTABLE: bool;
}

///
/// This type represents a const type.
/// 
pub struct Const;

impl self::seal::Sealed for Const { }
impl Mutability for Const {
    const IS_MUTABLE: bool = false;
}

///
/// This type represents a mutable type.
/// 
pub struct Mut;

impl self::seal::Sealed for Mut { }
impl Mutability for Mut {
    const IS_MUTABLE: bool = true;
}


pub(crate) const fn div_ceil(lhs: usize, rhs: usize) -> usize {
    let result = lhs / rhs;
    if (lhs % rhs) != 0 {
        result + 1

    } else {
        result
    }
}
