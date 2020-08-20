#![macro_use]


/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
macro_rules! c_str {
    ($literal:expr) => {
      unsafe {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
      }
    }
}

/// Get offset to struct member, similar to `offset_of` in C/C++
/// From https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    }
}

macro_rules! to_string {
  ($v:expr) => {
    format!("<{0:.2}, {1:.2}, {2:.2}>", $v.x, $v.y, $v.z)
  };
}