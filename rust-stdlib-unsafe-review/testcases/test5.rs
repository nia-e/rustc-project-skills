/// Reads one value from each pointer.
///
/// # Safety
///
/// For the duration of this call, each pointer must be aligned and valid for
/// reading one initialized `u32`. Neither pointee may be mutated concurrently.
pub unsafe fn read_pair(first: *const u32, second: *const u32) -> (u32, u32) {
    // SAFETY: `first` satisfies this function's caller contract.
    unsafe {
        let a = first.read();
        let b = second.read();
        (a, b)
    }
}
