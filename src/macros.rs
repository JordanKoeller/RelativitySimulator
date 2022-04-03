#![macro_use]

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
macro_rules! c_str {
    ($literal:expr) => {
        unsafe { CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes()) }
    };
}

macro_rules! to_string {
    ($v:expr) => {
        format!("<{0:.2}, {1:.2}, {2:.2}>", $v.x, $v.y, $v.z)
    };
}
