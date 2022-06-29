use std::mem::size_of;
use std::os::raw::c_void;

pub fn buff_sz<T>(data: &Vec<T>) -> isize {
    (size_of::<T>() * data.len()) as isize
}

pub fn buff_ptr<T>(data: &Vec<T>) -> *const c_void {
    &data[0] as *const T as *const c_void
}

pub unsafe fn cast_slice<F, T>(data: &[F]) -> &[T] {
    let raw_ptr = &data[0] as *const F as *const c_void as *const T;
    let old_sz = size_of::<F>();
    let new_sz = size_of::<T>();
    if new_sz < old_sz {
        std::slice::from_raw_parts(raw_ptr, data.len() * old_sz / new_sz)
    } else if old_sz < new_sz {
        std::slice::from_raw_parts(raw_ptr, data.len() * new_sz / old_sz)
    } else {
        std::slice::from_raw_parts(raw_ptr, data.len())
    }
}

pub unsafe fn cast_slice_mut<F, T>(data: &mut [F]) -> &mut [T] {
    let old_sz = size_of::<F>();
    let new_sz = size_of::<T>();
    let raw_ptr = &mut data[0] as *mut F as *mut c_void as *mut T;
    if new_sz < old_sz {
        std::slice::from_raw_parts_mut(raw_ptr, data.len() * old_sz / new_sz)
    } else if old_sz < new_sz {
        std::slice::from_raw_parts_mut(raw_ptr, data.len() * new_sz / old_sz)
    } else {
        std::slice::from_raw_parts_mut(raw_ptr, data.len())
    }
    // let sz_ratio = old_sz / new_sz;
}


pub unsafe fn unsafe_cast<F, T>(data: &F) -> &T {
    let raw_ptr = data as *const F as *const c_void as *const T;
    &*raw_ptr
} 
pub unsafe fn unsafe_cast_mut<F, T>(data: &mut F) -> &mut T {
    let raw_ptr = data as *mut F as *mut c_void as *mut T;
    &mut *raw_ptr
}