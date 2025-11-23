use alloc::{slice, vec::Vec};
use core::alloc::{Layout, LayoutError};
use esp_alloc::HEAP;

pub fn vec_into_iram<T>(size: usize) -> Result<Vec<T>, LayoutError> {
    let ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::Internal.into(),
            Layout::array::<T>(size)?,
        )
    };

    assert!(!ptr.is_null(), "Vec in IRAM Allocation failed");

    Ok(unsafe { Vec::from_raw_parts(ptr as *mut T, 0, size) })
}

pub fn vec_into_psram<T>(size: usize) -> Result<Vec<T>, LayoutError> {
    let ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::External.into(),
            Layout::array::<T>(size)?,
        )
    };

    assert!(!ptr.is_null(), "Vec in PSRAM Allocation failed");

    Ok(unsafe { Vec::from_raw_parts(ptr as *mut T, 0, size) })
}

pub fn buffer_into_iram<T>(size: usize) -> Result<*mut [T], LayoutError> {
    let ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::Internal.into(),
            Layout::array::<T>(size)?,
        )
    };

    assert!(!ptr.is_null(), "Buffer in IRAM Allocation failed");

    Ok(unsafe { slice::from_raw_parts_mut(ptr as *mut T, size) })
}

pub fn buffer_into_psram<T>(size: usize) -> Result<*mut [T], LayoutError> {
    let ptr = unsafe {
        HEAP.alloc_caps(
            esp_alloc::MemoryCapability::External.into(),
            Layout::array::<T>(size)?,
        )
    };

    assert!(!ptr.is_null(), "Buffer in PSRAM Allocation failed");

    Ok(unsafe { slice::from_raw_parts_mut(ptr as *mut T, size) })
}
