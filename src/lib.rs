//! This library overwrites the PE header in memory with nulls. This can trick some antivirus & analysis software. This library was not created with malicious intent but for educational purposes.
#![crate_name = "erase_pe_header"]

use std::ptr;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::RtlZeroMemory;

const PAGE_READWRITE: u32 = 0x04;
const OLD_PROTECT: u32 = 0;
const DWORD_SIZE: usize = 4096;

/// This function erases the portable-x86-executable header from file in memory
///
/// ## Example
///
/// ```rust
/// // This function erases the pe-header on startup
/// fn main() {
///    if cfg!(target_os = "windows") {
///        unsafe { erase_pe_header::erase_pe_header() };
///    }
///    println!("Hello world");
/// }
/// ```
#[cfg(target_os = "windows")]
pub unsafe fn erase_pe_header() {
    let old_protect = &mut OLD_PROTECT as *mut u32;
    let base_addr_ptr = GetModuleHandleA(ptr::null());
    let base_addr = base_addr_ptr as *mut winapi::ctypes::c_void;
    VirtualProtect(base_addr, DWORD_SIZE, PAGE_READWRITE, old_protect);
    RtlZeroMemory(base_addr, DWORD_SIZE);
}
