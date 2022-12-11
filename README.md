# EnumBitFlags
A proc-macro crate for Rust that allows creating bit flags enums.


How to use:
1. First you need to this crate to your `cargo.toml` file:
```toml
[dependencies]
EnumBitFlags = "0.1.0"
```

2. Then, you can use it in your Rust project like this:
```rs
#[EnumBitFlags]
enum MyFlags {
  Flag_1 = 0x0001,
  Flag_2 = 0x0002,
  Flag_3 = 0x0004
}

fn main() {
  let flags = MyFlags::Flag_1 | MyFlags::Flag_2;
}
```
