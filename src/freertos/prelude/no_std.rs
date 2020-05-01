pub use core::{cell::{RefCell, UnsafeCell},
               cmp::*,
               fmt,
               intrinsics::write_bytes,
               iter::Iterator,
               marker::PhantomData,
               mem,
               num::Wrapping,
               ops::{Deref, DerefMut, Range},
               prelude::*,
               ptr};

pub use alloc::{boxed::Box,
                rc::Rc,
                sync::{Arc, Weak}};

#[cfg(not(feature = "core_collections"))]
pub use alloc::string::*;
#[cfg(not(feature = "core_collections"))]
pub use alloc::vec::Vec;

#[cfg(feature = "core_collections")]
pub use collections::string::*;
#[cfg(feature = "core_collections")]
pub use collections::vec::Vec;
