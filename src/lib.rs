
pub mod bitmap;
pub mod slice;
pub mod store;
mod polyfill;

pub mod prelude {

    pub use crate::bitmap::Bitmap;
    pub use crate::slice::{
        BitmapSlice,
        BitmapSliceImpl,
        BitmapSliceIter,
        BitmapSliceMut,
        BitmapSliceRangeIter
    };

    pub use crate::store::BitStore;
    pub use crate::polyfill::{
        Const,
        Mut,
        Mutability
    };
}
