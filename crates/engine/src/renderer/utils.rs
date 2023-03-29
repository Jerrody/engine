#[inline(always)]
pub fn to_cstr<'a>(s: *const std::ffi::c_char) -> &'a std::ffi::CStr {
    unsafe { std::ffi::CStr::from_ptr(s) }
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        concat!($s, "\0").as_ptr().cast::<::std::os::raw::c_char>()
    };
}

pub use cstr;
