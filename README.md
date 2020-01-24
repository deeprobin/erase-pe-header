# erase-pe-header

This crate overwrites the PE header in memory with nulls. This can trick some antivirus & analysis software and it could be useful in malware creation. 
This crate was not created with malicious intent but for educational purposes. 
(Only works on windows)

This crate was adapted from [C++ Source](https://github.com/LordNoteworthy/al-khaser/blob/8ff90a3979face6e29aacb12521b032f2b379073/al-khaser/AntiDump/ErasePEHeaderFromMemory.cpp#L8-L22 "Repository LordNoteworthy/al-khasar").

## Example
```rust
fn main() {
    if cfg!(target_os = "windows") {
        unsafe { erase_pe_header::erase_pe_header() };
    }
    println!("Hello world");
}
```