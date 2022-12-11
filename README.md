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
  
  // check if a flag is set (via .contains(...) method)
  if flags.contains(MyFlags::Flag_1) {
    println!("Flag_1 is present");
  }
  
  // check if a flag is set (via AND operation)
  if (flags & MyFlags::Flag_2) == MyFlags::Flag_2 {
    println!("Flag_2 is present");
  }
}
```

# Arguments
EnumBitFlags supports various arguments that provide additional information on how to build the enum. Arguments are specified in the `EnumBitFlags` arguments with the following format: `key=value,key=value,...`. Alternativelly, you can use `:` instead of `=` (`key:value, key:value....`)

* `bits`  In-memory representation of the bitfield. It could be one of the following:`8`, `16`, `32`, `64` or `128`. If not specified the default value is `32`.
  _Example_
  ```rs
  #[EnumBitFlags(bits=8)]
  enum MyFlags {
    Flag_1 = 0x01,
    Flag_2 = 0x02,
    Flag_3 = 0x04
  }
  ```
* `empty` The name of the empty variant. An empty variant is the case where not bits are being set up. If not specified, it will `None` will be generated. The name of the empty variant must NOT be present in the enum variants and must start with a letter or underline character and can contain letters, numbers and the underline character. _Example_
  ```rs
  #[EnumBitFlags(empty=Nothing)]
  enum MyFlags {
    Flag_1 = 1,
    Flag_2 = 2,
    Flag_3 = 4
  }
  
  fn main() {
    let f = MyFlags::Nothing;
  }
  ```
  
# Methods
Every EnumBitFlags has several methods that can be used to easily manipulate and chek bits status:
* `contains` Checks if an exact bitflag mask is present
   ```rs
   fn contains(obj: <EnumName>) -> bool
   ```
   The obj must not be empty (at least one bit has to be set) and all bits from the object must be present.
   Example:
   ```rs
   #[EnumBitFlags]
   enum MyFlags { A = 1, B = 2, C = 4 }
  
   fn main() {
      let t = MyFlags::A | MyFlags::B;
      if t.contains(MyFlags::A) { 
        /* this code will be executed */ 
      }
      if t.contains(MyFlags::A | MyFlags::B) {
        /* this code will be executed */ 
      }
      if t.contains(MyFlags::A | MyFlags::C) {
        /* this code WILL NOT BE REACHED as flags C is not set in variable t */
      }
   }```
