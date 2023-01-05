# EnumBitFlags
A proc-macro crate for Rust that allows creating bit flags enums.


How to use:
1. First you need to this crate to your `cargo.toml` file:
```toml
[dependencies]
EnumBitFlags = "1.0.7"
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
* `empty` The name of the empty variant. An empty variant is the case where not bits are being set up. If not specified, `None` will be generated. The name of the empty variant must NOT be present in the enum variants and must start with a letter or underline character and can contain letters, numbers and the underline character. _Example_
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
  
* `disable_empty_generation` Disables the generation of an empty (value 0) variant. By default this is `None` but the name can be changed by using the `empty` attribute. This attribute will also disable any manual variant with value 0 (e.g. ```No_flag = 0```) _Example_
  ```rs
  #[EnumBitFlags(disable_empty_generation=true)]
  enum MyFlags {
    Flag_1 = 1,
    Flag_2 = 2,
    Flag_3 = 4
  }
  
  fn main() {
    let f = MyFlags::None; // this code will produce an error as variant None will not be generated
  }
  ```

  * `debug` Will print the resulted structure after parsing.  _Example_
  ```rs
  #[EnumBitFlags(debug=true)]
  enum MyFlags {
    Flag_1 = 1,
    Flag_2 = 2,
    Flag_3 = 4
  }
  ```

# Methods
Every EnumBitFlags has several methods that can be used to easily manipulate and chek bits status:

|Method                    |Description|
|--------------------------|-----------|
|**obj.contains(mask)**    |Returns `true` if all set bits from the mask are present in the object, or `false` otherwise|
|**obj.contains_one(mask)**|Returns `true` if at least one bit from the mask is present in the object, or `false` otherwise|
|**obj.clear()**           |Clears all bits from the current object|
|**obj.is_empty()**        |Returns `true` if not bits are set, `false` otherwise|
|**obj.remove(mask)**      |Removes all set flags from the mask|
|**obj.set(mask)**         |Set all bits from the mask|
|**obj.get_value()**       |Returns the numerical value associated to the bit mask flags|


* `contains` Checks if an exact bitflag mask is present
   ```rs
   fn contains(&self, obj: <EnumName>) -> bool
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
   }
  ```

* `contains_one` Checks if at least one bit from the mask is present in the object
   ```rs
   fn contains_one(&self, mask: <EnumName>) -> bool
   ```
   The obj must not be empty (at least one bit has to be set) .
   Example:
   ```rs
   #[EnumBitFlags]
   enum MyFlags { A = 1, B = 2, C = 4 }
  
   fn main() {
      let t = MyFlags::A | MyFlags::B;
      if t.contains_one(MyFlags::A) { 
        /* this code will be executed */ 
      }
      if t.contains_one(MyFlags::A | MyFlags::B) {
        /* this code will be executed */ 
      }
      if t.contains_one(MyFlags::A | MyFlags::C) {
        /* this code will be executed */
      }
   }
  ```
* `clear` Clears all bits from the enum
   ```rs
   fn clear(&mut self) 
   ```
   Example:
   ```rs
   #[EnumBitFlags]
   enum MyFlags { A = 1, B = 2, C = 4 }
  
   fn main() {
      let mut t = MyFlags::A | MyFlags::B;
      if t.contains_one(MyFlags::A) { 
        /* this code will be executed */ 
      }
      t.clear();
      if t.contains(MyFlags::A) {
        /* this code will NOT BE REACHED as t was cleared */ 
      }
   }
  ```
