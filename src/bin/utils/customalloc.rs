use alloc::vec::Vec;
use core::alloc::{Layout, LayoutError};
use esp_alloc::HEAP;

pub fn vec_into_iram<T>(size: usize) -> Result<Vec<T>, LayoutError> {
    let ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::Internal.into(),
            Layout::array::<T>(size)?,
        )
    };

    Ok(unsafe { Vec::from_raw_parts(ptr as *mut T, 0, size) })
}

pub fn vec_into_psram<T>(size: usize) -> Result<Vec<T>, LayoutError> {
    let ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::External.into(),
            Layout::array::<T>(size)?,
        )
    };

    Ok(unsafe { Vec::from_raw_parts(ptr as *mut T, 0, size) })
}
