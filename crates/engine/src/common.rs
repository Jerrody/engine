use std::hash::Hasher;

use once_cell::sync::OnceCell;

pub static mut HASHER: OnceCell<ahash::AHasher> = OnceCell::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id {
    hash: u64,
}

impl Id {
    #[inline(always)]
    pub fn new<H: std::hash::Hash>(value: H) -> Self {
        unsafe {
            value.hash(HASHER.get_mut().unwrap_unchecked());
            let hash = HASHER.get().unwrap_unchecked().finish();

            Self { hash }
        }
    }
}
